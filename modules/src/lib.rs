mod users {
    
    pub struct User{
        pub name: String
    }

}
mod Admins;

use crate::Admins::Admins::AddAdmin;
use crate::users::User;

fn dosomething(){
    let usrs = users::User{name:String::from("Ansh")};
    AddAdmin();
}