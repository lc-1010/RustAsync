use std::marker::PhantomPinned;
use std::pin::Pin;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Test{
    a:String,
    b:*const String,
}
impl Test{
    fn new(txt:&str)->Self{
        Test {
            a:String::from(txt),
            b:std::ptr::null(),
        }
    }
    fn init(&mut self){
        let self_ref:* const String= &self.a;
        self.b = self_ref;
    }
    fn a(&self)->&str{
        &self.a
    }
    fn b(&self)->&String{
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe{
            &*(self.b)
        }
    }
}

#[derive(Debug)]
struct PinTest{
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinTest{
    fn new(txt:&str)->Self{
        PinTest { 
            a: String::from(txt), 
            b: std::ptr::null(),
            _marker: PhantomPinned
            }
    }
    fn init(self: Pin<&mut Self>){
        let self_prt:*const String = &self.a;
        let this = unsafe {
            self.get_unchecked_mut()
        };
        this.b = self_prt;
    }

    fn a (self:Pin<&Self>)->&str{
        &self.get_ref().a
    }
    fn b(self:Pin<&Self>)->&String{
        assert!(!self.b.is_null(),"Test::b called without Test::init being called first");
        unsafe{
            &*(self.b)
        }
    }

}

#[derive(Debug)]
struct UpinTest{
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl UpinTest{
    fn new(txt:&str)->Pin<Box<Self>>{
        let t= UpinTest{
            a:String::from(txt),
            b:std::ptr::null(),
            _marker:PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe{
            boxed.as_mut().get_unchecked_mut().b = self_ptr
        };
        boxed
    }
    fn a(self:Pin<&Self>)->&str{
        &self.get_ref().a 
    }
    fn b(self:Pin<&Self>)->&String{
        unsafe{
            &*self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn no_pin_trait(){
        let mut test1 = Test::new("test1");
        test1.init();
        let mut test2 = Test::new("test2");
        test2.init();

        println!("a=>{} b => {}", test1.a(),test1.b());
        println!("a=>{} b => {}", test2.a(),test2.b());
        assert_eq!(test2.a(),test2.b(),"test1.a,test2.a");
        // std::mem::swap(&mut test1,&mut test2);
        // test1.a = "测试修改test1，和测试二".to_string();
        // println!("a=>{} b => {}", test2.a(),test2.b());
        // assert_eq!(test2.a(),test2.b(),"test1.a,test2.a");
    }
    #[test]
    fn use_pin_trait(){
        let mut test1 = PinTest::new("test1");
        let mut test1_pin= unsafe{
            Pin::new_unchecked(&mut test1)
        };
        PinTest::init(test1_pin.as_mut()); 
        // //dorp(test1_pin);
        // println!("{}",test1.as_ref().a());
        // assert_eq!(test1.as_ref().a(),test1.as_ref().b());
        
        let mut test2 = PinTest::new("test2");
        let mut test2 = unsafe{
          Pin::new_unchecked(&mut test2)  
        };
        PinTest::init(test2.as_mut());
        //std::mem::swap(test1.get_mut(),test2.get_mut());
    }

    #[test]
    fn pin_test(){
        let test1 = UpinTest::new("test1");
        let test2 = UpinTest::new("test2");
        println!("{}", test1.as_ref().a());
        assert_eq!(test1.as_ref().a(),test1.as_ref().b());
        assert_eq!(test2.as_ref().a(),test2.as_ref().b());

    }
}
