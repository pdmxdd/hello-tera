#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
//#[macro_use] extern crate tera;

//use rocket::http::RawStr;
//use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;
use rocket::request::Form;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    name: String
}


#[get("/")]
fn index() -> Redirect {
    Redirect::to("/login")
}



//#[get("/hello/<name>")]
//fn hello(name: &RawStr) -> String {
//    format!("Hello, {}!", name.as_str())
//}

#[get("/demo_template")]
fn demo_template() -> Template {
    let context = TemplateContext {
        title: String::from("TERA TEMPLATE"),
        name: String::from("Paul"),
        //items: vec!["Red", "Green", "Blue"].iter().map(|s| s.to_string()).collect()
    };
    Template::render("demo_template", &context)
}

#[derive(Serialize)]
struct BaseTemplateContext {
    title: String,
    logged_in: bool
}
#[get("/base")]
fn base() -> Template {
    /* 
        Following struct (context) is hard coded
        will need to be changed after we have the ORM, and DB working
    */
    let context = BaseTemplateContext {
        title: String::from("Base"),
        logged_in: false,
    };
    Template::render("base", &context)
}

#[get("/login")]
fn login() -> Template {
    /* 
        Following struct (context) is hard coded
        will need to be changed after we have the ORM, and DB working
    */
    let context = BaseTemplateContext {
        title: String::from("Login"),
        logged_in: true,
    };
    Template::render("login", &context)
}

#[derive(FromForm)]
struct LoginForm {
    email: String,
    password: String,
}

#[post("/login", data = "<loginform>")]
fn login_post(loginform: Form<LoginForm>) -> Redirect {
    /*
        This is just an example of how you get information out of a form
        you can then use that data however you want, more than likely it will go to the DB via ORM
    */
    println!("{}", loginform.get().email);
    Redirect::to("/base")
}

#[get("/logout")]
fn logout() -> Redirect {
    /* 
        We will need to put the logic for the logout here, before the redirect hits
        1. Delete any cookies
        2. Clear active user
        3. Redirect to user_login page (or something)
    */
    Redirect::to("/base")
}


fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        index,
        //hello,
        demo_template,
        base,
        login,
        login_post,
        logout,
    ])
    .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
