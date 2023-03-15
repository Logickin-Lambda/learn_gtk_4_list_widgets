mod imp;

use gtk::*;
use glib::*;


glib::wrapper!{
    pub struct StringObject(ObjectSubclass<imp::StringObject>);
}

impl StringObject{
    pub fn new(number: i32) -> Self{
        Object::builder().property("number", number).build()
    }
}
