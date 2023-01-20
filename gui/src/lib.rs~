// Listing 17-3: Definition of the Draw trait
pub trait Draw {
    fn draw(&self);
}

// Listing 17-4: Definition of the Screen struct with a components field holding a vector of trait objects that implement the Draw trait
pub struct Screen {
    // We create a trait object by specifying some sort of pointer, 
    // such as a & reference or a Box<T> smart pointer, 
    // then the dyn keyword
    pub components: Vec<Box<dyn Draw>>,
}

// Listing 17-5: A run method on Screen that calls the draw method on each component
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Listing 17-7: A Button struct that implements the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
