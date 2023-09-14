// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

///
/// Suppose
///     message Person {
///         repeated uint64 ids = 1;
///     }
///
/// We generate getter
///     Person::ids(&self)::get(&self, index: usize) -> Option<View<'a, uint64>>;
///
/// This should generate cpp (with a pre-cpp bounds check in rust to check for None case)
///     msg->ids(index)
///
/// However, the existing RawVTableMutator is implemented for existing primitive
/// types expecting that they are within a Message, not within a RepeatedField.
///
/// Thus we likely need new mutators for repeated fields that additionally take
/// the index as the argument.
///
/// DESIGN: cpp protobuf yields undefiend behavior for invalid indices, what should we do?
///
/// DESIGN: current proposal https://docs.google.com/document/d/1uL122khZplFNMCffaLhb2WKejtTxun3UlDxKK5aI_tY/edit?resourcekey=0-21rSGlDN51w9kBkeT_DgZg
///     suggests returning a `View<'_, T>` to a specific index of a repeated
///     field. However, View<'_, T> is already implemented as a field-accessor
///     that holds a Message pointer.
///     IDEA 1: create RepeatedItemView/Mut<'_, T> which holds
///     RepeatedField pointer _and_ the index, allowing the same get and set API
///     as View/Mut.
///     IDEA 2: RawVTableMutator could hold an enum of MessageRef OR RepeatedField+index, and require that the VTable have get_at and set_at methods
use std::marker::PhantomData;

use crate::{Proxied, SettableValue, View, ViewProxy, __internal::RawRepeatedField};

#[derive(Clone, Copy)]
pub struct RepeatedFieldRef<'a> {
    pub repeated_field: RawRepeatedField,
    _phantom: PhantomData<&'a mut ()>,
}

unsafe impl<'a> Send for RepeatedFieldRef<'a> {}
unsafe impl<'a> Sync for RepeatedFieldRef<'a> {}

pub struct Repeated<T: Proxied + ?Sized> {
    inner: RawRepeatedField,
    _phantom: PhantomData<*const T>,
}

#[derive(Clone, Copy)]
pub struct RepeatedView<'a, T: ?Sized> {
    inner: RepeatedFieldRef<'a>,
    _phantom: PhantomData<*const T>,
}

impl<'a, T: Proxied + ?Sized> RepeatedView<'a, T> {
    pub fn get(&self, i: usize) -> Option<T::View<'a>> {
        todo!()
    }
}

//  I don't think this impl makes sense, protoc doesn't generate this for cpp (setting one RepeatedField from another RepeatedField)
// impl<T: Proxied + ?Sized> SettableValue<Repeated<T>> for RepeatedView<'_, T> {
//     fn set_on(self, _private: crate::__internal::Private, mutator: crate::Mut<'_, Repeated<T>>) {
//         todo!()
//     }
// }

impl<'a, T: Proxied> std::fmt::Debug for RepeatedView<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RepeatedView").finish()
    }
}

pub struct RepeatedMut<'a, T: ?Sized> {
    _asd: PhantomData<&'a T>,
}

//  in line with comment above, this impl doesn't make sense, we might need a whole set of traits for repeated fields
// impl<T: Proxied + ?Sized> Proxied for Repeated<T>
// where
//     T: Send,
// {
//     type View<'a> = RepeatedView<'a, T>
//     where
//         Self: 'a;

//     type Mut<'a> = RepeatedMut<'a, T>
//     where
//         Self: 'a;
// }
