use std::cell::Cell;

use gtk::{glib::once_cell::sync::Lazy, prelude::ToValue};
use gtk::subclass::prelude::*;
//use gtk::prelude::*;
use gtk::glib;
use glib::{ParamSpecString, ParamSpec, Value};


// Object holding the state
#[derive(Default)]
pub struct StringObject {
    string: Cell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for StringObject{
    const NAME: &'static str = "CustomStringObject";
    type Type = super::StringObject;
}

impl ObjectImpl for StringObject{
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = 
            Lazy::new(|| vec![
                ParamSpecString::builder("string").build(), 
                ParamSpecString::builder("sunvox").build(), // we can define multiple properties here since it is a vec! macro
                ParamSpecString::builder("notey").build()   
            ]);
            PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name(){
            "string" => {
                let input_string = value.get().expect("Must be a string");
                self.string.replace(input_string);       
            }
            "sunvox" => {
                self.string.replace("The Best Music Tracker".to_string());
            }
            "notey" => {
                self.string.replace("ANSWER!!! PLEASE!!!".to_string());
            }
            _ => unimplemented!() 
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name(){
            "string" => {
                self.string.take().to_value()   // instead of using get() in the i32 example, seems String use take()
            }
            _ => unimplemented!() 
        }
    }
}