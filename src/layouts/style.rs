pub type Style<T: Stylable> = fn(T)->T;

pub trait Stylable {}

pub trait AddStyle<T> {
    fn style (self, style: Style<Self>) -> T;
}
