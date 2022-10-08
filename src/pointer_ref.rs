// Reference Pointers - Point to a resource in memory

pub fn run() {
   // primitive array
   let arr1 = [1,2,3];
   let arr2 = arr1;

   //println!("values: {:?}", (arr1, arr2));

   //with non-primitives, if you assign another variable to a piece of data, the first
   //variable will no longer hold that value. You'll need to use a reference (&) to point yo
   //the resource

   //vector is not primitive!!! you can copy it. reference should be copied
   let vec1 = vec![1,2,3];
   let vec2 = &vec1;

   println!("values: {:?}", (&vec1, vec2));

}