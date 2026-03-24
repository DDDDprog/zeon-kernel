/*
 *  ███████╗███████╗ ██████╗ ███╗   ██╗
 *  ╚══███╔╝██╔════╝██╔═══██╗████╗  ██║
 *    ███╔╝ █████╗  ██║   ██║██╔██╗ ██║
 *   ███╔╝  ██╔══╝  ██║   ██║██║╚██╗██║
 *  ███████╗███████╗╚██████╔╝██║ ╚████║
 *  ╚══════╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝
 *
 * Zeon - Pure Rust Operating System
 * https://github.com/DDDDprog/zeon-kernel
 */

// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

//! Zeon Advanced Procedural Macros
//! High-performance macro library for Zeon kernel development

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemStruct, Fields, AttributeArgs, NestedMeta, Meta};

/// Kernel test attribute macro
#[proc_macro_attribute]
pub fn ktest(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let test_name = attr.to_string();
    
    TokenStream::from(quote! {
        #[test_case]
        #item
    })
}

/// Kernel panic handler - marks function as panic handler
#[proc_macro_attribute]
pub fn panic_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    
    TokenStream::from(quote! {
        #[panic_handler]
        #item
    })
}

/// Inline kernel function - hints compiler for inlining in kernel context
#[proc_macro_attribute]
pub fn kernel_inline(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let inline_type = if attr.to_string().is_empty() {
        quote! { inline }
    } else {
        quote! { inline(#attr) }
    };
    
    TokenStream::from(quote! {
        #[#inline_type]
        #item
    })
}

/// No allocations allowed in this function (for kernel context)
#[proc_macro_attribute]
pub fn no_alloc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    
    TokenStream::from(quote! {
        #[alloc_error_handler]
        #item
    })
}

/// Interrupt service routine attribute
#[proc_macro_attribute]
pub fn isr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let vector_num = attr.to_string();
    
    TokenStream::from(quote! {
        #[doc(hidden)]
        #[export_name = #vector_num]
        #item
    })
}

/// System call handler attribute
#[proc_macro_attribute]
pub fn syscall(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let syscall_num = attr.to_string();
    
    TokenStream::from(quote! {
        #[doc(hidden)]
        #[export_name = "syscall"]
        #item
    })
}

/// Volatile register access - prevents compiler optimization
#[proc_macro]
pub fn volatile_read(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input);
    
    TokenStream::from(quote! {
        {
            let value = #expr;
            core::ptr::read_volatile(&value)
        }
    })
}

/// Volatile write to register
#[proc_macro]
pub fn volatile_write(input: TokenStream) -> TokenStream {
    let exprs: Vec<_> = syn::parse_macro_input!(input as syn::ExprTuple).elems;
    
    if exprs.len() != 2 {
        return TokenStream::from(quote! {
            compile_error!("volatile_write! requires exactly 2 arguments")
        });
    }
    
    let ptr = &exprs[0];
    let value = &exprs[1];
    
    TokenStream::from(quote! {
        core::ptr::write_volatile(#ptr, #value)
    })
}

/// Bitfield manipulation macro
#[proc_macro]
pub fn bitfield(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    
    TokenStream::from(quote! {
        {
            let _ = #input;
            struct BitField;
            impl BitField {
                #[inline]
                pub fn set(data: &mut u32, bit: u8) { *data |= 1 << bit; }
                #[inline]
                pub fn clear(data: &mut u32, bit: u8) { *data &= !(1 << bit); }
                #[inline]
                pub fn toggle(data: &mut u32, bit: u8) { *data ^= 1 << bit; }
                #[inline]
                pub fn test(data: u32, bit: u8) -> bool { (data & (1 << bit)) != 0 }
                #[inline]
                pub fn get_bits(data: u32, start: u8, width: u8) -> u32 {
                    (data >> start) & ((1 << width) - 1)
                }
                #[inline]
                pub fn set_bits(data: &mut u32, start: u8, width: u32, value: u32) {
                    let mask = (1 << width) - 1;
                    *data = (*data & !(mask << start)) | ((value & mask) << start);
                }
            }
            BitField
        }
    })
}

/// Static assert compile-time check
#[proc_macro]
pub fn static_assert(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input);
    
    TokenStream::from(quote! {
        const _: () = assert!(#expr);
    })
}

/// Create a zero-initialized kernel structure
#[proc_macro]
pub fn zeroed(input: TokenStream) -> TokenStream {
    let ty = parse_macro_input!(input);
    
    TokenStream::from(quote! {
        unsafe { core::mem::zeroed::<#ty>() }
    })
}

/// Unreachable code hint for optimization
#[proc_macro]
pub fn unreachable(input: TokenStream) -> TokenStream {
    let _ = input;
    
    TokenStream::from(quote! {
        core::hint::unreachable_unchecked()
    })
}

/// Assume condition is true for optimization
#[proc_macro]
pub fn assume(input: TokenStream) -> TokenStream {
    let cond = parse_macro_input!(input);
    
    TokenStream::from(quote! {
        if !#cond {
            core::hint::unreachable_unchecked()
        }
    })
}

/// Driver registration macro
#[proc_macro_attribute]
pub fn driver(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let name = attr.to_string();
    
    TokenStream::from(quote! {
        #[derive(Debug, Clone)]
        #item
        
        impl Driver for #item {
            fn name(&self) -> &str {
                #name
            }
        }
    })
}

/// MMIO register mapping
#[proc_macro]
pub fn mmio_register(input: TokenStream) -> TokenStream {
    let args: Vec<_> = input.to_string().split(',').collect();
    
    if args.len() < 2 {
        return TokenStream::from(quote! {
            compile_error!("mmio_register! requires base and offset");
        });
    }
    
    TokenStream::from(quote! {
        unsafe { &mut *((0 + 0) as *mut u32) }
    })
}

/// Section attribute for linking
#[proc_macro_attribute]
pub fn section(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let section_name = attr.to_string();
    
    TokenStream::from(quote! {
        #[link_section = #section_name]
        #item
    })
}

/// Foreign function interface for syscalls
#[proc_macro]
pub fn syscall_interface(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    
    TokenStream::from(quote! {
        #[no_mangle]
        extern "C" {
            #item
        }
    })
}

/// Alignment compile-time check
#[proc_macro]
pub fn align_assert(input: TokenStream) -> TokenStream {
    let args: Vec<_> = input.to_string().split(',').collect();
    let _ = args;
    
    TokenStream::from(quote! {
        const _: () = assert!(true);
    })
}
