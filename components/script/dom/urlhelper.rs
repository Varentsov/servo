/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use servo_util::str::DOMString;
use url::Url;

pub struct UrlHelper;

impl UrlHelper {
    pub fn Href(url: &Url) -> DOMString {
        url.serialize()
    }

    pub fn Search(url: &Url) -> DOMString {
        match url.query {
            None => "".into_string(),
            Some(ref query) if query.as_slice() == "" => "".into_string(),
            Some(ref query) => format!("?{}", query)
        }
    }

    pub fn Hash(url: &Url) -> DOMString {
        match url.fragment {
            None => "".into_string(),
            Some(ref hash) if hash.as_slice() == "" => "".into_string(),
            Some(ref hash) => format!("#{}", hash)
        }
    }

    /// https://html.spec.whatwg.org/multipage/browsers.html#same-origin
    pub fn SameOrigin(urlA: &Url, urlB: &Url) -> bool {
        if urlA.host() != urlB.host() {
            return false
        }
        if urlA.scheme != urlB.scheme {
            return false
        }
        if urlA.port() != urlB.port() {
            return false
        }
        return true
    }
}
