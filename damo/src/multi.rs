use std::path::Path;

use crate::{CardView, DamoResult, Id, MemProvider, Provider, RedProvider};

use self::MPShape::{Mem, Red};

#[derive(Debug)]
pub struct MultiProvider(MPInner);

#[derive(Debug)]
pub struct MPCardScan<'a>(CSInner<'a>);

type MPInner = MPShape<MemProvider, RedProvider>;

type CSInner<'a> =
    MPShape<<MemProvider as Provider>::CardScan<'a>, <RedProvider as Provider>::CardScan<'a>>;

#[derive(Debug)]
enum MPShape<M, R> {
    Mem(M),
    Red(R),
}

impl MultiProvider {
    pub fn open_or_create<P>(optpath: Option<P>) -> DamoResult<Self>
    where
        P: AsRef<Path>,
    {
        MPShape::open_or_create(optpath).map(MultiProvider)
    }
}

impl MPInner {
    fn open_or_create<P>(optpath: Option<P>) -> DamoResult<Self>
    where
        P: AsRef<Path>,
    {
        if let Some(p) = optpath {
            RedProvider::open_or_create(p).map(Red)
        } else {
            Ok(Mem(MemProvider::default()))
        }
    }
}

impl Provider for MultiProvider {
    fn is_empty(&self) -> DamoResult<bool> {
        self.0.is_empty()
    }

    fn card_new(&mut self) -> DamoResult<Id> {
        self.0.card_new()
    }

    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> DamoResult<()> {
        self.0.card_set_synopsis(card, synopsis)
    }

    type CardScan<'a>
        = MPCardScan<'a>
    where
        Self: 'a;

    fn card_scan(&self) -> DamoResult<Self::CardScan<'_>> {
        self.0.card_scan().map(MPCardScan)
    }
}

impl Provider for MPInner {
    fn is_empty(&self) -> DamoResult<bool> {
        match self {
            Mem(x) => x.is_empty(),
            Red(x) => x.is_empty(),
        }
    }

    fn card_new(&mut self) -> DamoResult<Id> {
        match self {
            Mem(x) => x.card_new(),
            Red(x) => x.card_new(),
        }
    }

    fn card_set_synopsis(&mut self, card: Id, synopsis: &str) -> DamoResult<()> {
        match self {
            Mem(x) => x.card_set_synopsis(card, synopsis),
            Red(x) => x.card_set_synopsis(card, synopsis),
        }
    }

    type CardScan<'a>
        = CSInner<'a>
    where
        Self: 'a;

    fn card_scan(&self) -> DamoResult<Self::CardScan<'_>> {
        match self {
            Mem(x) => x.card_scan().map(Mem),
            Red(x) => x.card_scan().map(Red),
        }
    }
}

impl<'a> Iterator for MPCardScan<'a> {
    type Item = DamoResult<CardView<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<M, R, I> Iterator for MPShape<M, R>
where
    M: Iterator<Item = I>,
    R: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Mem(x) => x.next(),
            Red(x) => x.next(),
        }
    }
}
