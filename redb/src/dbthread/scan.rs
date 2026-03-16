use std::sync::mpsc;

use redb::{ReadTransaction, ReadableTable as _};

use crate::DbResult;
use crate::dbthread::Handler;
use crate::messages::{CardScan, CardScanned};
use crate::tables::TABLES;

#[derive(Debug, Default)]
pub(super) struct ScanSlot(Option<Inner>);

impl Handler<(ReadTransaction, CardScan)> for &mut ScanSlot {
    type Reply = CardScanned;

    fn handle(self, (txn, op): (ReadTransaction, CardScan)) -> DbResult<Self::Reply> {
        let inner = self.0.take().map(Ok).unwrap_or_else(|| Inner::init(txn))?;
        match inner.handle(op) {
            Ok(reply) => {
                self.0 = Some(inner);
                Ok(reply)
            }
            Err(e) => {
                inner.join_handle.join().unwrap()?;
                Err(e)
            }
        }
    }
}

#[derive(Debug)]
struct Inner {
    join_handle: std::thread::JoinHandle<DbResult<()>>,
    to_worker: mpsc::Sender<CardScan>,
    from_worker: mpsc::Receiver<CardScanned>,
}

impl Inner {
    fn init(txn: ReadTransaction) -> DbResult<Self> {
        let (to_worker, from_db) = mpsc::channel();
        let (to_db, from_worker) = mpsc::channel();
        let join_handle = std::thread::Builder::new()
            .name("scan".to_string())
            .spawn(|| run_worker(from_db, to_db, txn))
            .unwrap();

        Ok(Inner {
            join_handle,
            to_worker,
            from_worker,
        })
    }
}

impl Handler<CardScan> for &Inner {
    type Reply = CardScanned;

    fn handle(self, req: CardScan) -> DbResult<Self::Reply> {
        self.to_worker.send(req)?;
        let reply = self.from_worker.recv()?;
        Ok(reply)
    }
}

fn run_worker(
    from_db: mpsc::Receiver<CardScan>,
    to_db: mpsc::Sender<CardScanned>,
    txn: ReadTransaction,
) -> DbResult<()> {
    use CardScan::*;
    use CardScanned::*;

    let tab = txn.open_table(TABLES.card_synopsis)?;
    let iter = tab.iter()?.rev();

    for kvres in iter {
        let (reply, do_stop) = match from_db.recv()? {
            Next => {
                let (_, vg) = kvres?;
                let synopsis = vg.value().to_string();
                (Found(synopsis), false)
            }
            Stop => (Stopped, true),
        };

        to_db.send(reply)?;

        if do_stop {
            return Ok(());
        }
    }

    to_db.send(Ended)?;
    Ok(())
}
