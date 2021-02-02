pub mod attribute_set
{
use std::string::String;
use std::rc::Rc;
use std::collections::HashMap;
use std::any::Any;
use std::option::Option;

pub trait AttributeSetTrait
{
    fn put<T:'static>(&mut self, key: &str, value: Rc<T>);

    fn get<T:'static>(&self, key: &str) -> Option<&T>;
}


pub struct AttribteSet
{
    map: HashMap<String, Rc<dyn Any>>
}

impl AttribteSet
{
    pub fn new() -> Self
    {
        AttribteSet { map: HashMap::new() }
    }
}



impl AttributeSetTrait for AttribteSet
{
    fn put<T:'static>(&mut self, key: &str, value: Rc<T>)
    {
        self.map.entry(key.to_string()).or_insert(value);
    }

    fn get<T:'static>(&self, key: &str) -> Option<&T>
    {
        let rtn:Option<&Rc<dyn Any>> = self.map.get(key);
        match rtn {
            None => None,
            Some(refRc) => refRc.downcast_ref::<T>()
        }
    }
}

}

#[cfg(test)]
mod tests {

    use super::attribute_set::*;
    use std::rc::Rc;
    use std::any::Any;
    #[test]
    fn downcast_my() {
        let val = Rc::new(Rc::new(5));
        let y: Rc<Any> = val;
        let z = y.downcast_ref::<Rc<i32>>();
        match z {
            None => assert!(false),
            Some(g) => println!("succ {}", g)
        };
    }

    #[test]
    fn it_works() {
        let mut attr = AttribteSet::new();
        attr.put::<i32>("5", Rc::new(5));
        attr.put::<String>("str", Rc::new(String::from("12")));
        let irefopt = attr.get::<i32>("5");
        if let(Some(irc)) = irefopt
        {
            assert_eq!(*irc, 5);
            println!("succ {}", irc);
        }  
        else
        {
            println!("none");
            assert!(false);
        }
        let strrefopt = attr.get::<String>("str");
        if let(Some(strrc)) = strrefopt
        {
            println!("{}", strrc);
        }
        else
        {
            println!("none");
            assert!(false);
        }
    }
}
