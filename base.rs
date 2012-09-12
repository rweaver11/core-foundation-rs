use libc::c_long;

pub type Boolean = u8;

pub type CFIndex = c_long;

struct __CFAllocator { private: () }
pub type CFAllocatorRef = *__CFAllocator;

struct __CFType { private: () }
pub type CFTypeRef = *__CFType;

struct CFType {
    obj: CFTypeRef,

    drop {
        unsafe {
            CFRelease(self.obj)
        }
    }
}

impl CFType : AbstractCFType {
    pure fn as_type_ref(&self) -> CFTypeRef {
        self.obj
    }
}

trait AbstractCFType {
    pure fn as_type_ref(&self) -> CFTypeRef;
}

trait CFTypeOps {
    fn as_type(&self) -> CFType;
    fn retain_count(&self) -> CFIndex;
    fn show(&self);
}

impl<T:AbstractCFType> T : CFTypeOps {
    // FIXME: Should move, but there's a linearity bug.
    fn as_type(&self) -> CFType {
        CFRetain(self.as_type_ref());
        CFType { obj: self.as_type_ref() }
    }

    fn retain_count(&self) -> CFIndex {
        CFGetRetainCount(self.as_type_ref())
    }

    fn show(&self) {
        CFShow(self.as_type_ref());
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFBase.h
     */

    const kCFAllocatorDefault: CFAllocatorRef;
    const kCFAllocatorSystemDefault: CFAllocatorRef;
    const kCFAllocatorMalloc: CFAllocatorRef;
    const kCFAllocatorMallocZone: CFAllocatorRef;
    const kCFAllocatorNull: CFAllocatorRef;
    const kCFAllocatorUseContext: CFAllocatorRef;

    fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    fn CFRelease(cf: CFTypeRef);

    /*
     * CFString.h
     */

    fn CFShow(obj: CFTypeRef);
}
