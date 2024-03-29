use std::{ffi::CString, ptr};

use jdk_sys::{jfieldID, jmethodID, jvalue, JNI_TRUE};

use crate::{env::Jenv, object::JObject, unchecked_jnic, unchecked_jnice, jvalue::JValue};
#[derive(Debug, Clone,Copy)]
pub struct JClass<'a> {
    pub ptr : jdk_sys::jclass,
    pub env : &'a Jenv<'a>,
}
impl<'a> JClass<'a> {
    pub const fn new(ptr:jdk_sys::jclass,env : &'a Jenv) -> Self {
        JClass {
            ptr,
            env,
        }
    }
    pub fn find(env : &'a Jenv,clz:&str) -> Result<Self,()> {
        env.find_class(clz)
    }
    pub fn null(env : &'a Jenv) -> Self {
        JClass {
            ptr : ptr::null_mut(),
            env
        }
    }
    pub fn get_super<T: From<JObject<'a>>>(&self) -> Result<JClass<'a>,()> {
        let obj = unchecked_jnic!(self.env.ptr,GetSuperclass, self.ptr);
        if obj.is_null() {
            return Err(());
        }

        Ok(JClass::new(obj,self.env))
    }
    pub fn new_object<T : From<JObject<'a>>>(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<T,()> {
        let mid=  self.get_method_id(name,sig)?;
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();

        let obj = unchecked_jnic!(self.env.ptr,NewObjectA, self.ptr, mid,args.as_ptr());
        if obj.is_null() {
            return Err(());
        }
        Ok(T::from(JObject::new(obj,self.env)))
    }

    pub fn get_field_id(&self, name:&str, sig:&str) -> Result<jfieldID,()> {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetFieldID, self.ptr, name.as_ptr(), sig.as_ptr())

    }
    pub fn get_static_field_id(&self, name:&str, sig:&str) -> Result<jfieldID,()> {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetStaticFieldID, self.ptr, name.as_ptr(), sig.as_ptr())
        
    }
    pub fn get_method_id(&self, name:&str, sig:&str) -> Result<jmethodID,()> {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetMethodID, self.ptr, name.as_ptr(), sig.as_ptr())

    }
    pub fn get_static_method_id(&self, name:&str, sig:&str) -> Result<jmethodID,()> {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();
        unchecked_jnice!(self.env.ptr,GetStaticMethodID, self.ptr, name.as_ptr(), sig.as_ptr())
        
    }

    // static methods

    pub fn call_static_object_method<T:From<JObject<'a>>>(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();

        let mid=  self.get_static_method_id(name,sig)?;
        obj = unchecked_jnice!(self.env.ptr,CallStaticObjectMethodA, self.ptr, mid,args.as_ptr())?;
        if obj.is_null() {
            return Err(());
        }

        Ok(T::from(JObject::new(obj,self.env)))
    }
    pub fn call_static_boolean_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<bool,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        Ok(unchecked_jnice!(self.env.ptr,CallStaticBooleanMethodA, self.ptr, mid,args.as_ptr() )? == JNI_TRUE as u8)
    }
    pub fn call_static_byte_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i8,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallStaticByteMethodA, self.ptr, mid,args.as_ptr() )
    }
    pub fn call_static_char_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<char,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        Ok(unchecked_jnice!(self.env.ptr,CallStaticCharMethodA, self.ptr, mid,args.as_ptr())? as u8 as char)
    }
    pub fn call_static_short_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i16,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallStaticShortMethodA, self.ptr, mid,args.as_ptr() )
    }
    pub fn call_static_int_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i32,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallStaticIntMethodA, self.ptr, mid,args.as_ptr() )
    }
    pub fn call_static_long_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<i64,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallStaticLongMethodA, self.ptr, mid,args.as_ptr() )
    }
    pub fn call_static_float_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<f32,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallStaticFloatMethodA, self.ptr, mid,args.as_ptr() )
    }
    pub fn call_static_double_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<f64,()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallStaticDoubleMethodA, self.ptr, mid,args.as_ptr() )
    }
    pub fn call_static_void_method(&self,name:&str,sig:&str,args:&Vec<JValue>) -> Result<(),()> {
        let args = args.iter().map(|f|f.get_c_style()).collect::<Vec<jvalue>>();
        let mid=  self.get_static_method_id(name,sig)?;

        unchecked_jnice!(self.env.ptr,CallStaticVoidMethodA, self.ptr, mid,args.as_ptr() )
    }

    // static fields

    // get

    pub fn get_static_object_field<T: From<JObject<'a>>>(&self,name:&str,sig:&str) -> Result<T,()> {
        let mut obj = ptr::null_mut();
        let fid = self.get_static_field_id(name,sig)?;
        obj = unchecked_jnice!(self.env.ptr,GetStaticObjectField, self.ptr, fid)?;
    
        if obj.is_null() {
            return Err(());
        }
        Ok(T::from(JObject::new(obj,self.env)))
    }

    pub fn get_static_boolean_field(&self,name:&str,sig:&str) -> Result<bool,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticBooleanField, self.ptr, fid)? == JNI_TRUE as u8)
    }
    pub fn get_static_byte_field(&self,name:&str,sig:&str) -> Result<i8,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticByteField, self.ptr, fid)? as i8)
    }
    pub fn get_static_char_field(&self,name:&str,sig:&str) -> Result<char,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticCharField, self.ptr, fid)? as u8 as char)
    }
    pub fn get_static_short_field(&self,name:&str,sig:&str) -> Result<i16,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticShortField, self.ptr, fid)?)
    }
    pub fn get_static_int_field(&self,name:&str,sig:&str) -> Result<i32,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticIntField, self.ptr, fid)?)
    }
    pub fn get_static_long_field(&self,name:&str,sig:&str) -> Result<i64,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticLongField, self.ptr, fid)?)
    }
    pub fn get_static_float_field(&self,name:&str,sig:&str) -> Result<f32,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticFloatField, self.ptr, fid)?)
    }
    pub fn get_static_double_field(&self,name:&str,sig:&str) -> Result<f64,()> {
        let fid = self.get_static_field_id(name,sig)?;
        Ok(unchecked_jnice!(self.env.ptr,GetStaticDoubleField, self.ptr, fid)?)
    }

    // set

    pub fn set_static_object_field(&self,name:&str,sig:&str,new_value:&'a JObject<'a>) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticObjectField, self.ptr, fid,new_value.ptr)
    }

    pub fn set_static_boolean_field(&self,name:&str,sig:&str, new_value:bool) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticBooleanField, self.ptr, fid,new_value as u8)
    }
    pub fn set_static_byte_field(&self,name:&str,sig:&str,new_value:i8) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticByteField, self.ptr, fid,new_value as i8)
    }
    pub fn set_static_char_field(&self,name:&str,sig:&str,new_value:char) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticCharField, self.ptr, fid, new_value as u16)
    }
    pub fn set_static_short_field(&self,name:&str,sig:&str,new_value:i16) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticShortField, self.ptr, fid,new_value)
    }
    pub fn set_static_int_field(&self,name:&str,sig:&str,new_value:i32) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticIntField, self.ptr, fid,new_value)
    }
    pub fn set_static_long_field(&self,name:&str,sig:&str,new_value:i64) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticLongField, self.ptr, fid,new_value)
    }
    pub fn set_static_float_field(&self,name:&str,sig:&str,new_value:f32) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticFloatField, self.ptr, fid,new_value)
    }
    pub fn set_static_double_field(&self,name:&str,sig:&str,new_value:f64) -> Result<(),()> {
        let fid = self.get_static_field_id(name,sig)?;
        unchecked_jnice!(self.env.ptr,SetStaticDoubleField, self.ptr, fid,new_value)
    }
    


    // util

    pub fn get_name() {}

}

pub trait JClassExt<T> {
    fn set_static_field(cls:&JClass, name: &str, sig: &str, new_value: T) -> Result<(), ()>;
    fn get_static_field(cls:&JClass, name: &str, sig: &str) -> Result<T, ()>;
    fn call_static_method(cls:&JClass, name: &str, sig: &str, args:&Vec<JValue>) -> Result<T, ()>;
}

jni_proc::generate_jclass_impls!(bool,boolean);
jni_proc::generate_jclass_impls!(i8,byte);
jni_proc::generate_jclass_impls!(char,char);
jni_proc::generate_jclass_impls!(i16,short);
jni_proc::generate_jclass_impls!(i32,int);
jni_proc::generate_jclass_impls!(i64,long);
jni_proc::generate_jclass_impls!(f32,float);
jni_proc::generate_jclass_impls!(f64,double);






impl<'a> From<JObject<'a>> for JClass<'a> {
    fn from(x: JObject<'a>) -> Self {
        Self::new(x.ptr, x.env)
    }
}