use super::{Body, BodyStream, BoxBody, BoxError, Collect, Data, Limited, MapErr, Next};

pub trait BodyExt: Body {
    fn next(&mut self) -> Next<'_, Self>
    where
        Self: Unpin,
    {
        Next(self)
    }

    fn data(&mut self) -> Data<'_, Self>
    where
        Self: Unpin,
    {
        Data(self)
    }

    fn map_err<F, E>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> E,
    {
        MapErr::new(self, f)
    }

    fn limit(self, limit: usize) -> Limited<Self>
    where
        Self: Sized,
    {
        Limited::new(self, limit)
    }

    fn collect(self) -> Collect<Self>
    where
        Self: Sized,
    {
        Collect::new(self)
    }

    fn stream(self) -> BodyStream<Self>
    where
        Self: Sized,
    {
        BodyStream::new(self)
    }

    fn boxed(self) -> BoxBody
    where
        Self: Sized + Send + 'static,
        Self::Error: Into<BoxError>,
    {
        super::try_downcast(self).unwrap_or_else(BoxBody::new)
    }
}

impl<B> BodyExt for B where B: Body {}
