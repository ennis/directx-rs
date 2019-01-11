use com_wrapper::ComWrapper;
use std::ops::Deref;

pub unsafe fn wrap_com<T>(ptr: *mut T::Interface) -> T
where
    T: ComWrapper,
{
    assert!(!ptr.is_null());
    T::from_raw(ptr)
}

pub unsafe fn wrap_opt_com<T>(ptr: *mut T::Interface) -> Option<T>
where
    T: ComWrapper,
{
    if !ptr.is_null() {
        Some(T::from_raw(ptr))
    } else {
        None
    }
}

pub unsafe fn wrap_ref_to_raw_com<T>(ptr: &*mut T::Interface) -> &T
where
    T: ComWrapper,
{
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>()
    );
    assert!(!ptr.is_null());
    std::mem::transmute::<&*mut _, &T>(ptr)
}

pub unsafe fn wrap_ref_to_raw_mut_com<T>(ptr: &mut *mut T::Interface) -> &mut T
where
    T: ComWrapper,
{
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>()
    );
    assert!(!ptr.is_null());
    std::mem::transmute::<&mut *mut _, &mut T>(ptr)
}

pub unsafe fn wrap_opt_ref_to_raw_com<T>(ptr: &*mut T::Interface) -> Option<&T>
where
    T: ComWrapper,
{
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>(),
    );
    if ptr.is_null() {
        None
    } else {
        Some(std::mem::transmute::<&*mut _, &T>(ptr))
    }
}

pub unsafe fn unwrap_opt_com<T>(com: Option<&T>) -> *mut T::Interface
where
    T: ComWrapper,
{
    com.map(|i| i.get_raw()).unwrap_or(std::ptr::null_mut())
}

pub unsafe fn deref_com_wrapper<T, U>(wrapper: &T) -> &U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(std::mem::size_of::<U>(), std::mem::size_of::<T>());

    std::mem::transmute::<&T, &U>(wrapper)
}

pub unsafe fn deref_com_wrapper_mut<T, U>(wrapper: &mut T) -> &mut U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(std::mem::size_of::<U>(), std::mem::size_of::<T>());

    std::mem::transmute::<&mut T, &mut U>(wrapper)
}
