// function_probes

//#[allow(unused_imports)]
use crate::display_functions::{
  show_op, show_type, show_value, shows, putln,
  show_label
};
use std::fmt::Debug;

/*-----------------------------------------------
   Pass by value moves the argument into the
   function's stack frame, so invalid after call
*/
pub fn pass_by_value<T>(mut t:T) where T:Debug + Default {
  show_op("in pass_by_value");
  show_type(&t);
  show_value(&t);
  print!("  t = T::default()\n");
  t = T::default();
  show_value(&t);
  show_op("leaving function");
}
/*-----------------------------------------------
   Pass by ref borrows the argument. Borrow
   moved into the function's stack frame.  
   Borrow ends at end of function call, so 
   param is valid after call
*/
pub fn pass_by_ref<T: std::default::Default + Debug>(rt:&mut T) {
  show_op("in pass_by_ref");
  show_type(&rt);
  show_value(&rt);
  /*-- demonstrate side effects --*/
  show_op("*rt = T::default()");
  *rt = T::default();
  show_value(&rt);
  show_op("leaving function");
}
/*-----------------------------------------------
   Pass by ref change instance
   borrows the first argument and moves the second argument
   Borrow ends at end of function call, so 
   rt is valid after call
*/
pub fn pass_move_by_ref_change_value<T: From<String> + Debug>(rt:&mut T) 
{
  show_op("in pass_by_ref_change_value");
  show_type(&rt);
  show_value(&rt);
  print!("  address of rt is: {:p}\n", &rt);
  /*-- demonstrate side effects --*/
  *rt = "vec![3, 2, 1]".to_string().into();  // into type T
  print!("  address of rt is: {:p}\n", &rt);
  show_value(&rt);
  show_op("leaving function");
}
/*-----------------------------------------------
   Pass move by ref change instance
     borrows argument
     Borrow ends at end of function call, so 
     rh is valid after call
*/
pub fn pass_move_by_ref_change_heap_instance<T>(rh:&mut Box<T>)
  where T: From<String> + Default + Debug 
{
  show_op("in pass_move_by_ref_change_heap_instance");
  show_type(&rh);
  show_value(&rh);
  print!("  address of rh is: {:p}\n", *rh);
  /*---------------------------------------
    demonstrate side effects by changing
    referenced object 
  */
  show_op("*rh = Box::new(t)");
  let t:T = "a new string".to_string().into();
  *rh = Box::new(t);
  print!("  address of rh is: {:p}\n", *rh);
  show_value(&rh);
  show_op("leaving function");
}

pub fn demo_pass_by() {

  show_label("pass copy type by value, caller doesn't see any changes");
  let d = 3.14159;
  pass_by_value(d);
  print!("  value of d is now {d}\n");
  assert_eq!(d, 3.14159);
  shows("  no side-effects, passing copy type by value\n");
  putln();

  show_label("pass copy type by value, caller doesn't see any changes");
  let s = "a string";  // literal is copy type
  pass_by_value(s);
  print!("  value of s is now {s:?}\n");
  assert_eq!(s, "a string");
  shows("  no side-effects, passing copy type by value\n");
  putln();

  show_label("pass copy type by ref, caller sees any changes");
  let mut s = "a string";  // literal is copy type
  pass_by_ref(&mut s);
  print!("  value of s is now {s:?}\n");
  assert_eq!(s, "");
  shows("  side-effects, passing copy type by ref\n");
  putln();

  show_label("pass copy type by ref, caller sees change of value");
  let mut d = 3.1415927;
  pass_by_ref(&mut d);
  assert_ne!(d, 3.1415927);
  print!("  d now has value: {d:?}\n");
  shows("  side-effects, pass copy type by ref and change value\n");
  putln();

  show_label("pass move type by value, caller sees invalidation");
  let s = "a string".to_string();  // move type
  pass_by_value(s);
  // statement below fails to compile: s moved
  // print!("{s}\n");
  shows("  can't access s, been moved\n");
  shows("  side-effects, passing move type by value\n");
  putln();

  show_label("pass move type by ref, caller sees change of value");
  let mut s = "a string".to_string();  // move type
  pass_by_ref(&mut s);
  assert_eq!(s, "".to_string());
  print!("  s now has value: {s:?}\n");
  shows("  side-effects, passing move type by ref\n");
  putln();

  show_label("pass move type by ref, caller sees change of value");
  let mut v = vec![1, 2, 3];  // move type
  pass_by_ref(&mut v);
  //assert_ne!(v, "a string".to_string());
  print!("  v now has value: {v:?}\n");
  shows("  side-effects, pass move type by ref and change value\n");
  putln();

  // show_label("pass move type by ref, caller sees change of value");
  // let mut v = vec![1, 2, 3];  // move type
  // pass_move_by_ref_change_value(&mut v);
  // //assert_ne!(v, "a string".to_string());
  // print!("  v now has value: {v:?}\n");
  // shows("  side-effects, pass move type by ref and change value\n");
  // putln();

  show_label("pass move type by ref, caller sees change of value");
  let mut s = "a string".to_string();  // move type
  pass_move_by_ref_change_value(&mut s);
  assert_ne!(s, "a string".to_string());
  print!("  s now has value: {s:?}\n");
  shows("  side-effects, pass move type by ref and change value\n");
  putln();
  
  show_label("pass move type by ref, caller sees change of instance");
  let s = "a string".to_string();       // move type
  let mut smrt_ptr_heap = Box::new(s);  // s moved into Box
  pass_move_by_ref_change_heap_instance(&mut smrt_ptr_heap);
  assert_ne!(*smrt_ptr_heap, "a string".to_string());
  print!("  smrt_ptr_heap now has value: {:?}\n", *smrt_ptr_heap);
  shows("  side-effects, pass move type by ref and change instance");
  putln();
}