use crate::{Value, unsafe_bindings};
use std::time::{Duration, SystemTime};

const MAC_EPOCH: u64 = 978307200; // 01/01/2001

crate::impl_node!(
    /// A date plist node.
    ///
    /// Represents a time passed since the Unix Epoch.
    Date
);

impl Date<'_> {
    /// Creates a new date plist node.
    ///
    /// The duration must represent a time passed since the Unix Epoch.
    ///
    /// Note: the original library expects you to pass the number of seconds
    /// since 01/01/2001 (Mac Epoch). You **don't** need to do this here.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use plist_plus2::Date;
    ///
    /// let some_date = Date::new(Duration::from_secs(1546635600));
    /// let now: Date = SystemTime::now().into();
    /// ```
    pub fn new(date: Duration) -> Self {
        // The number of seconds since 01/01/2001
        let mac_epoch = Duration::from_secs(MAC_EPOCH);
        let secs = (date.as_secs() as i64 - mac_epoch.as_secs() as i64) as i32;
        let usecs = (((date.as_micros() as i64 - mac_epoch.as_micros() as i64) as i128)
            - (secs as i128 * 1000000)) as i32;

        let ptr = unsafe { unsafe_bindings::plist_new_date(secs, usecs) };
        Self {
            pointer: ptr,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns a duration (a Unix Timestamp) of the date.
    pub fn get(&self) -> Duration {
        let mut sec = unsafe { std::mem::zeroed() };
        let mut usec = unsafe { std::mem::zeroed() };
        unsafe { unsafe_bindings::plist_get_date_val(self.pointer, &mut sec, &mut usec) };
        let date = usec as i64 + (sec as i64) * 1000000;
        Duration::from_micros((MAC_EPOCH as i64 * 1000000 + date) as u64)
    }

    /// Sets the date with a Unix Timestamp.
    ///
    /// The duration must represent a time passed since the Unix Epoch.
    pub fn set(&mut self, date: Duration) {
        let mac_epoch = Duration::from_secs(MAC_EPOCH);
        let secs = (date.as_secs() as i64 - mac_epoch.as_secs() as i64) as i32;
        let usecs = (((date.as_micros() as i64 - mac_epoch.as_micros() as i64) as i128)
            - (secs as i128 * 1000000)) as i32;
        unsafe { unsafe_bindings::plist_set_date_val(self.pointer, secs, usecs) };
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'b>(&self) -> Date<'b> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.pointer) };
        (unsafe { crate::from_pointer(pointer) })
            .into_date()
            .unwrap()
    }
}

impl From<Duration> for Date<'_> {
    fn from(value: Duration) -> Self {
        Date::new(value)
    }
}

impl From<SystemTime> for Date<'_> {
    fn from(value: SystemTime) -> Self {
        // unwrapping is safe since UNIX_EPOCH is always lesser than SystemTime::now()
        Date::new(value.duration_since(SystemTime::UNIX_EPOCH).unwrap())
    }
}

impl From<Duration> for Value<'_> {
    fn from(value: Duration) -> Self {
        Date::from(value).into()
    }
}

impl From<SystemTime> for Value<'_> {
    fn from(value: SystemTime) -> Self {
        Date::from(value).into()
    }
}

impl From<Date<'_>> for Duration {
    fn from(value: Date<'_>) -> Self {
        value.get()
    }
}

impl PartialEq for Date<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl Default for Date<'_> {
    fn default() -> Self {
        Duration::default().into()
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Date<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn check_unix_mac_date() {
        let timestamp = 1546635600123456; // Jan 04 2019 21:00:00.123456

        let unix_date = Duration::from_micros(timestamp);
        let unix_plist = Date::new(unix_date);

        let secs = 1546635600 - MAC_EPOCH;
        let usecs = 123456;

        let mac_plist =
            unsafe { crate::from_pointer(unsafe_bindings::plist_new_date(secs as i32, usecs)) };

        assert_eq!(unix_plist.get(), mac_plist.as_date().unwrap().get());
    }

    #[test]
    fn date_before_mac_epoch() {
        let duration = Duration::from_secs(358860726); // 16 May 1981 at 15:32:06
        let date = Date::new(duration.clone());
        assert_eq!(duration, date.get());
    }

    #[test]
    fn set_random_date() {
        let timestamp = 1546635600123456; // Jan 04 2019 21:00:00.123456

        let date = Duration::from_micros(timestamp);
        let mut plist: Date<'_> = SystemTime::now().into(); // create a new date with a current time
        plist.set(date); // set a new time

        assert_eq!(date, plist.get());
    }
}
