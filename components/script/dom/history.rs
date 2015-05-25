/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HistoryBinding;
use dom::bindings::codegen::Bindings::HistoryBinding::HistoryMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, JSRef, Rootable, Temporary};
use dom::bindings::utils::{Reflector, reflect_dom_object};
use dom::window::Window;
use dom::window::WindowHelpers;

// https://html.spec.whatwg.org/multipage/browsers.html#history-3
#[dom_struct]
pub struct History {
    reflector_: Reflector,
    window: JS<Window>,
}

impl History {
    fn new_inherited(window: JSRef<Window>) -> History {
        History {
            reflector_: Reflector::new(),
            window: JS::from_rooted(window)
        }
    }

    pub fn new(window: JSRef<Window>) -> Temporary<History> {
        reflect_dom_object(box History::new_inherited(window),
                           GlobalRef::Window(window),
                           HistoryBinding::Wrap)
    }
}

impl<'a> HistoryMethods for JSRef<'a, History> {
    // https://html.spec.whatwg.org/multipage/browsers.html#dom-history-back
    fn Back(self) {
        
    }
    
    // https://html.spec.whatwg.org/multipage/browsers.html#dom-history-forward
    fn Forward(self) {
        
    }
}


