mod string_object;

use gtk::{*, prelude::*};

const APP_LIST  : &str = "app_list";

fn main() -> glib::ExitCode{
    
    // build two apps according to the gtk tutorial, default list widget and the custom list widget that can hide element 
    let app = Application::builder().application_id(APP_LIST).build();

    app.connect_activate(build_simple_list);
    app.connect_activate(build_custom_list);

    app.run()
}

fn build_simple_list(app: &Application){

    // craete a list box for showing a list of number:
    let list_box = ListBox::new();  

    // generate string from aa to zz, as I want to try other type instead of number based on the tutorial
    for charset_a in 'a'..='z'{
        for charset_b in 'a'..='z'{
            let label = Label::new(Some(format!("{0}{1}", &charset_a, &charset_b).as_str()));
            // we can append items like a vec
            list_box.append(&label);
        }
    }
    // scrolled_window is necessary for large or long widgets; 
    // otherwise, the list will expend the windows indefinitely until it hit the screen border, 
    // and it will not display the item located beyond the screen.
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        // .vscrollbar_policy(PolicyType::Automatic)    // it is auto by default
        .min_content_height(360)
        .min_content_width(360)    // it doesn't work if the scrollerbar policy is set to NEVER
        .child(&list_box)
        .build();

    let window = Window::builder()
        .application(app)
        .default_width(360)
        .default_height(360)
        .title("Standard List Box Element")
        .child(&scrolled_window)
        .build();

    window.present();

}

fn build_custom_list(app: &Application){
    
    // generate a vec with character aaaa to zzzz
    let string_vec = generate_all_four_letter_combinations();

    // create a string model
    let model = gio::ListStore::new(StringObject::static_type());

    // add the string vec into the model
    model.extend_from_slice(&string_vec);

    // This factory is used for emit signal for each item in the list
    let factory = SignalListItemFactory::new();

    // This will emit a signal when a listitem has been created, normally being the first emitted signal;
    // for emitting signal on destroy, use connect_teardown
    factory.connect_setup(move |_, list_item|{
        let label = Label::new(None);
        list_item
            // promote the object class to the parent class
            .downcast_ref::<ListItem>()
            .expect("Must be a list item")
            .set_child(Some(&label));
    });

    // Still understanding why we need this function
    // TODO: study the purpose of this function:
    // 
    factory.connect_bind(move |_, list_item| {
        // get the StringObject from the list item
        let string_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Must be a list item")
            .item() // get the display item from the list item
            .and_downcast::<StringObject>()
            .expect("Must be a StringObject");

        // extract the string from the StringObject
        let string = string_object.property::<String>("string");

        // get the label from the list item
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Must be a list item")
            .child() // get the widget from the list item
            .and_downcast::<Label>()
            .expect("Must be a Label");

        // fitting the string into the label
        label.set_label(&string);
    });

    // Do a multiple selection for the list object
    let selection_model = MultiSelection::new(Some(model));

    // this function is used for displaying a list dynamically
    let list_view = ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        // .vscrollbar_policy(PolicyType::Automatic)    // it is auto by default
        .min_content_height(360)
        .min_content_width(360)    // it doesn't work if the scrollerbar policy is set to NEVER
        .child(&list_view)
        .build();

    let window = Window::builder()
        .application(app)
        .default_width(360)
        .default_height(360)
        .title("Standard List Box Element")
        .child(&scrolled_window)
        .build();

    window.present();

}

fn generate_all_four_letter_combinations() -> Vec<StringObject>{

    let mut char_vec = Vec::new();

    // bad nesting here, but I want all combination from aaaa - zzzz.
    for char_a in 'a'..'z'{
        for char_b in 'a'..'z'{
            for char_c in 'a'..'z'{
                for char_d in 'a'..'z'{
                    let string_object = StringObject::new(format!("{0}{1}{2}{3}", char_a, char_b, char_c, char_d).as_str());
                    char_vec.push(string_object);
                }
            }
        }
    }

    char_vec
}