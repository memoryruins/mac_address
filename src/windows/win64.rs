#![allow(bad_style)]
#![allow(dead_code)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }

    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                val |= 1 << i;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            self.set_bit(i + bit_offset, val_bit_is_set);
        }
    }
}
pub type wchar_t = ::std::os::raw::c_ushort;
pub type ULONG = ::std::os::raw::c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BYTE = ::std::os::raw::c_uchar;
pub type INT = ::std::os::raw::c_int;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type UINT32 = ::std::os::raw::c_uint;
pub type ULONG64 = ::std::os::raw::c_ulonglong;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type CHAR = ::std::os::raw::c_char;
pub type WCHAR = wchar_t;
pub type PWCHAR = *mut WCHAR;
pub type PCHAR = *mut CHAR;
pub type ULONGLONG = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GUID {
    pub Data1: ::std::os::raw::c_ulong,
    pub Data2: ::std::os::raw::c_ushort,
    pub Data3: ::std::os::raw::c_ushort,
    pub Data4: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout__GUID() {
    assert_eq!(
        ::std::mem::size_of::<_GUID>(),
        16usize,
        concat!("Size of: ", stringify!(_GUID))
    );
    assert_eq!(
        ::std::mem::align_of::<_GUID>(),
        4usize,
        concat!("Alignment of ", stringify!(_GUID))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data2 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data3 as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data4 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data4)
        )
    );
}
pub type GUID = _GUID;
pub type ADDRESS_FAMILY = USHORT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: ADDRESS_FAMILY,
    pub sa_data: [CHAR; 14usize],
}
#[test]
fn bindgen_test_layout_sockaddr() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(sockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_data as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_data)
        )
    );
}
pub type LPSOCKADDR = *mut sockaddr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SOCKET_ADDRESS {
    pub lpSockaddr: LPSOCKADDR,
    pub iSockaddrLength: INT,
}
#[test]
fn bindgen_test_layout__SOCKET_ADDRESS() {
    assert_eq!(
        ::std::mem::size_of::<_SOCKET_ADDRESS>(),
        16usize,
        concat!("Size of: ", stringify!(_SOCKET_ADDRESS))
    );
    assert_eq!(
        ::std::mem::align_of::<_SOCKET_ADDRESS>(),
        8usize,
        concat!("Alignment of ", stringify!(_SOCKET_ADDRESS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SOCKET_ADDRESS>())).lpSockaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SOCKET_ADDRESS),
            "::",
            stringify!(lpSockaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SOCKET_ADDRESS>())).iSockaddrLength as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_SOCKET_ADDRESS),
            "::",
            stringify!(iSockaddrLength)
        )
    );
}
pub type SOCKET_ADDRESS = _SOCKET_ADDRESS;
pub type IFTYPE = ULONG;
pub type NET_IF_COMPARTMENT_ID = UINT32;
pub type NET_IF_NETWORK_GUID = GUID;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _NET_LUID_LH {
    pub Value: ULONG64,
    pub Info: _NET_LUID_LH__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NET_LUID_LH__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize], u32>,
    pub __bindgen_align: [u64; 0usize],
}
#[test]
fn bindgen_test_layout__NET_LUID_LH__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_NET_LUID_LH__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_NET_LUID_LH__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_NET_LUID_LH__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_NET_LUID_LH__bindgen_ty_1))
    );
}
impl _NET_LUID_LH__bindgen_ty_1 {
    #[inline]
    pub fn Reserved(&self) -> ULONG64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 24u8) as u64) }
    }
    #[inline]
    pub fn set_Reserved(&mut self, val: ULONG64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub fn NetLuidIndex(&self) -> ULONG64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 24u8) as u64) }
    }
    #[inline]
    pub fn set_NetLuidIndex(&mut self, val: ULONG64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub fn IfType(&self) -> ULONG64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(48usize, 16u8) as u64) }
    }
    #[inline]
    pub fn set_IfType(&mut self, val: ULONG64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(48usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        Reserved: ULONG64,
        NetLuidIndex: ULONG64,
        IfType: ULONG64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 24u8, {
            let Reserved: u64 = unsafe { ::std::mem::transmute(Reserved) };
            Reserved as u64
        });
        __bindgen_bitfield_unit.set(24usize, 24u8, {
            let NetLuidIndex: u64 = unsafe { ::std::mem::transmute(NetLuidIndex) };
            NetLuidIndex as u64
        });
        __bindgen_bitfield_unit.set(48usize, 16u8, {
            let IfType: u64 = unsafe { ::std::mem::transmute(IfType) };
            IfType as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout__NET_LUID_LH() {
    assert_eq!(
        ::std::mem::size_of::<_NET_LUID_LH>(),
        8usize,
        concat!("Size of: ", stringify!(_NET_LUID_LH))
    );
    assert_eq!(
        ::std::mem::align_of::<_NET_LUID_LH>(),
        8usize,
        concat!("Alignment of ", stringify!(_NET_LUID_LH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_NET_LUID_LH>())).Value as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_NET_LUID_LH),
            "::",
            stringify!(Value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_NET_LUID_LH>())).Info as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_NET_LUID_LH),
            "::",
            stringify!(Info)
        )
    );
}
pub type NET_LUID_LH = _NET_LUID_LH;
pub type NET_LUID = NET_LUID_LH;
pub type IF_LUID = NET_LUID;
pub type NET_IFINDEX = ULONG;
pub type IF_INDEX = NET_IFINDEX;
pub const _NET_IF_CONNECTION_TYPE_NET_IF_CONNECTION_DEDICATED: _NET_IF_CONNECTION_TYPE = 1;
pub const _NET_IF_CONNECTION_TYPE_NET_IF_CONNECTION_PASSIVE: _NET_IF_CONNECTION_TYPE = 2;
pub const _NET_IF_CONNECTION_TYPE_NET_IF_CONNECTION_DEMAND: _NET_IF_CONNECTION_TYPE = 3;
pub const _NET_IF_CONNECTION_TYPE_NET_IF_CONNECTION_MAXIMUM: _NET_IF_CONNECTION_TYPE = 4;
pub type _NET_IF_CONNECTION_TYPE = i32;
pub use self::_NET_IF_CONNECTION_TYPE as NET_IF_CONNECTION_TYPE;
pub const TUNNEL_TYPE_TUNNEL_TYPE_NONE: TUNNEL_TYPE = 0;
pub const TUNNEL_TYPE_TUNNEL_TYPE_OTHER: TUNNEL_TYPE = 1;
pub const TUNNEL_TYPE_TUNNEL_TYPE_DIRECT: TUNNEL_TYPE = 2;
pub const TUNNEL_TYPE_TUNNEL_TYPE_6TO4: TUNNEL_TYPE = 11;
pub const TUNNEL_TYPE_TUNNEL_TYPE_ISATAP: TUNNEL_TYPE = 13;
pub const TUNNEL_TYPE_TUNNEL_TYPE_TEREDO: TUNNEL_TYPE = 14;
pub const TUNNEL_TYPE_TUNNEL_TYPE_IPHTTPS: TUNNEL_TYPE = 15;
pub type TUNNEL_TYPE = i32;
pub const IF_OPER_STATUS_IfOperStatusUp: IF_OPER_STATUS = 1;
pub const IF_OPER_STATUS_IfOperStatusDown: IF_OPER_STATUS = 2;
pub const IF_OPER_STATUS_IfOperStatusTesting: IF_OPER_STATUS = 3;
pub const IF_OPER_STATUS_IfOperStatusUnknown: IF_OPER_STATUS = 4;
pub const IF_OPER_STATUS_IfOperStatusDormant: IF_OPER_STATUS = 5;
pub const IF_OPER_STATUS_IfOperStatusNotPresent: IF_OPER_STATUS = 6;
pub const IF_OPER_STATUS_IfOperStatusLowerLayerDown: IF_OPER_STATUS = 7;
pub type IF_OPER_STATUS = i32;
pub const NL_PREFIX_ORIGIN_IpPrefixOriginOther: NL_PREFIX_ORIGIN = 0;
pub const NL_PREFIX_ORIGIN_IpPrefixOriginManual: NL_PREFIX_ORIGIN = 1;
pub const NL_PREFIX_ORIGIN_IpPrefixOriginWellKnown: NL_PREFIX_ORIGIN = 2;
pub const NL_PREFIX_ORIGIN_IpPrefixOriginDhcp: NL_PREFIX_ORIGIN = 3;
pub const NL_PREFIX_ORIGIN_IpPrefixOriginRouterAdvertisement: NL_PREFIX_ORIGIN = 4;
pub const NL_PREFIX_ORIGIN_IpPrefixOriginUnchanged: NL_PREFIX_ORIGIN = 16;
pub type NL_PREFIX_ORIGIN = i32;
pub const NL_SUFFIX_ORIGIN_NlsoOther: NL_SUFFIX_ORIGIN = 0;
pub const NL_SUFFIX_ORIGIN_NlsoManual: NL_SUFFIX_ORIGIN = 1;
pub const NL_SUFFIX_ORIGIN_NlsoWellKnown: NL_SUFFIX_ORIGIN = 2;
pub const NL_SUFFIX_ORIGIN_NlsoDhcp: NL_SUFFIX_ORIGIN = 3;
pub const NL_SUFFIX_ORIGIN_NlsoLinkLayerAddress: NL_SUFFIX_ORIGIN = 4;
pub const NL_SUFFIX_ORIGIN_NlsoRandom: NL_SUFFIX_ORIGIN = 5;
pub const NL_SUFFIX_ORIGIN_IpSuffixOriginOther: NL_SUFFIX_ORIGIN = 0;
pub const NL_SUFFIX_ORIGIN_IpSuffixOriginManual: NL_SUFFIX_ORIGIN = 1;
pub const NL_SUFFIX_ORIGIN_IpSuffixOriginWellKnown: NL_SUFFIX_ORIGIN = 2;
pub const NL_SUFFIX_ORIGIN_IpSuffixOriginDhcp: NL_SUFFIX_ORIGIN = 3;
pub const NL_SUFFIX_ORIGIN_IpSuffixOriginLinkLayerAddress: NL_SUFFIX_ORIGIN = 4;
pub const NL_SUFFIX_ORIGIN_IpSuffixOriginRandom: NL_SUFFIX_ORIGIN = 5;
pub const NL_SUFFIX_ORIGIN_IpSuffixOriginUnchanged: NL_SUFFIX_ORIGIN = 16;
pub type NL_SUFFIX_ORIGIN = i32;
pub const NL_DAD_STATE_NldsInvalid: NL_DAD_STATE = 0;
pub const NL_DAD_STATE_NldsTentative: NL_DAD_STATE = 1;
pub const NL_DAD_STATE_NldsDuplicate: NL_DAD_STATE = 2;
pub const NL_DAD_STATE_NldsDeprecated: NL_DAD_STATE = 3;
pub const NL_DAD_STATE_NldsPreferred: NL_DAD_STATE = 4;
pub const NL_DAD_STATE_IpDadStateInvalid: NL_DAD_STATE = 0;
pub const NL_DAD_STATE_IpDadStateTentative: NL_DAD_STATE = 1;
pub const NL_DAD_STATE_IpDadStateDuplicate: NL_DAD_STATE = 2;
pub const NL_DAD_STATE_IpDadStateDeprecated: NL_DAD_STATE = 3;
pub const NL_DAD_STATE_IpDadStatePreferred: NL_DAD_STATE = 4;
pub type NL_DAD_STATE = i32;
pub use self::NL_PREFIX_ORIGIN as IP_PREFIX_ORIGIN;
pub use self::NL_SUFFIX_ORIGIN as IP_SUFFIX_ORIGIN;
pub use self::NL_DAD_STATE as IP_DAD_STATE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_UNICAST_ADDRESS_LH {
    pub __bindgen_anon_1: _IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_UNICAST_ADDRESS_LH,
    pub Address: SOCKET_ADDRESS,
    pub PrefixOrigin: IP_PREFIX_ORIGIN,
    pub SuffixOrigin: IP_SUFFIX_ORIGIN,
    pub DadState: IP_DAD_STATE,
    pub ValidLifetime: ULONG,
    pub PreferredLifetime: ULONG,
    pub LeaseLifetime: ULONG,
    pub OnLinkPrefixLength: UINT8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub Flags: DWORD,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>()))
                .Length as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>()))
                .Flags as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Flags)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1>())).Alignment
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_UNICAST_ADDRESS_LH() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_UNICAST_ADDRESS_LH>(),
        64usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_UNICAST_ADDRESS_LH>(),
        8usize,
        concat!("Alignment of ", stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).Next as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).Address as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(Address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).PrefixOrigin as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(PrefixOrigin)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).SuffixOrigin as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(SuffixOrigin)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).DadState as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(DadState)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).ValidLifetime as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(ValidLifetime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).PreferredLifetime as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(PreferredLifetime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).LeaseLifetime as *const _
                as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(LeaseLifetime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_UNICAST_ADDRESS_LH>())).OnLinkPrefixLength
                as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_UNICAST_ADDRESS_LH),
            "::",
            stringify!(OnLinkPrefixLength)
        )
    );
}
pub type PIP_ADAPTER_UNICAST_ADDRESS_LH = *mut _IP_ADAPTER_UNICAST_ADDRESS_LH;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_ANYCAST_ADDRESS_XP {
    pub __bindgen_anon_1: _IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub Address: SOCKET_ADDRESS,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub Flags: DWORD,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>()))
                .Length as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>()))
                .Flags as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Flags)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1>())).Alignment
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ANYCAST_ADDRESS_XP() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ANYCAST_ADDRESS_XP>(),
        32usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ANYCAST_ADDRESS_XP>(),
        8usize,
        concat!("Alignment of ", stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ANYCAST_ADDRESS_XP>())).Next as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ANYCAST_ADDRESS_XP>())).Address as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ANYCAST_ADDRESS_XP),
            "::",
            stringify!(Address)
        )
    );
}
pub type PIP_ADAPTER_ANYCAST_ADDRESS_XP = *mut _IP_ADAPTER_ANYCAST_ADDRESS_XP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_MULTICAST_ADDRESS_XP {
    pub __bindgen_anon_1: _IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub Address: SOCKET_ADDRESS,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub Flags: DWORD,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>()))
                .Length as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>()))
                .Flags as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Flags)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1>())).Alignment
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_MULTICAST_ADDRESS_XP() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_MULTICAST_ADDRESS_XP>(),
        32usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_MULTICAST_ADDRESS_XP>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_MULTICAST_ADDRESS_XP>())).Next as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_MULTICAST_ADDRESS_XP>())).Address as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_MULTICAST_ADDRESS_XP),
            "::",
            stringify!(Address)
        )
    );
}
pub type PIP_ADAPTER_MULTICAST_ADDRESS_XP = *mut _IP_ADAPTER_MULTICAST_ADDRESS_XP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    pub __bindgen_anon_1: _IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub Address: SOCKET_ADDRESS,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub Reserved: DWORD,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(
            ))).Length as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1>(
            ))).Reserved as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Reserved)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1>())).Alignment
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_DNS_SERVER_ADDRESS_XP() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP>(),
        32usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP>())).Next as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_DNS_SERVER_ADDRESS_XP>())).Address as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_DNS_SERVER_ADDRESS_XP),
            "::",
            stringify!(Address)
        )
    );
}
pub type PIP_ADAPTER_DNS_SERVER_ADDRESS_XP = *mut _IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    pub __bindgen_anon_1: _IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub Address: SOCKET_ADDRESS,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub Reserved: DWORD,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(
            ))).Length as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(
            ))).Reserved as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Reserved)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1>())).Alignment
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_WINS_SERVER_ADDRESS_LH() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH>(),
        32usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH>())).Next as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_WINS_SERVER_ADDRESS_LH>())).Address as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_WINS_SERVER_ADDRESS_LH),
            "::",
            stringify!(Address)
        )
    );
}
pub type PIP_ADAPTER_WINS_SERVER_ADDRESS_LH = *mut _IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_GATEWAY_ADDRESS_LH {
    pub __bindgen_anon_1: _IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Address: SOCKET_ADDRESS,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub Reserved: DWORD,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>()))
                .Length as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1>()))
                .Reserved as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Reserved)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1>())).Alignment
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_GATEWAY_ADDRESS_LH() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_GATEWAY_ADDRESS_LH>(),
        32usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_GATEWAY_ADDRESS_LH>(),
        8usize,
        concat!("Alignment of ", stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_GATEWAY_ADDRESS_LH>())).Next as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_GATEWAY_ADDRESS_LH>())).Address as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_GATEWAY_ADDRESS_LH),
            "::",
            stringify!(Address)
        )
    );
}
pub type PIP_ADAPTER_GATEWAY_ADDRESS_LH = *mut _IP_ADAPTER_GATEWAY_ADDRESS_LH;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_PREFIX_XP {
    pub __bindgen_anon_1: _IP_ADAPTER_PREFIX_XP__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_PREFIX_XP,
    pub Address: SOCKET_ADDRESS,
    pub PrefixLength: ULONG,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_PREFIX_XP__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub Flags: DWORD,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1>())).Length
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1>())).Flags
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_PREFIX_XP__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Flags)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_PREFIX_XP__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_PREFIX_XP__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_PREFIX_XP__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_PREFIX_XP__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_PREFIX_XP__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_PREFIX_XP__bindgen_ty_1>())).Alignment as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_PREFIX_XP__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_PREFIX_XP() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_PREFIX_XP>(),
        40usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_PREFIX_XP))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_PREFIX_XP>(),
        8usize,
        concat!("Alignment of ", stringify!(_IP_ADAPTER_PREFIX_XP))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_PREFIX_XP>())).Next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_PREFIX_XP),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_PREFIX_XP>())).Address as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_PREFIX_XP),
            "::",
            stringify!(Address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_PREFIX_XP>())).PrefixLength as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_PREFIX_XP),
            "::",
            stringify!(PrefixLength)
        )
    );
}
pub type PIP_ADAPTER_PREFIX_XP = *mut _IP_ADAPTER_PREFIX_XP;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_DNS_SUFFIX {
    pub Next: *mut _IP_ADAPTER_DNS_SUFFIX,
    pub String: [WCHAR; 256usize],
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_DNS_SUFFIX() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_DNS_SUFFIX>(),
        520usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_DNS_SUFFIX))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_DNS_SUFFIX>(),
        8usize,
        concat!("Alignment of ", stringify!(_IP_ADAPTER_DNS_SUFFIX))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_DNS_SUFFIX>())).Next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_DNS_SUFFIX),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_DNS_SUFFIX>())).String as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_DNS_SUFFIX),
            "::",
            stringify!(String)
        )
    );
}
pub type PIP_ADAPTER_DNS_SUFFIX = *mut _IP_ADAPTER_DNS_SUFFIX;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IP_ADAPTER_ADDRESSES_LH {
    pub __bindgen_anon_1: _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1,
    pub Next: *mut _IP_ADAPTER_ADDRESSES_LH,
    pub AdapterName: PCHAR,
    pub FirstUnicastAddress: PIP_ADAPTER_UNICAST_ADDRESS_LH,
    pub FirstAnycastAddress: PIP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: PIP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: PWCHAR,
    pub Description: PWCHAR,
    pub FriendlyName: PWCHAR,
    pub PhysicalAddress: [BYTE; 8usize],
    pub PhysicalAddressLength: ULONG,
    pub __bindgen_anon_2: _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2,
    pub Mtu: ULONG,
    pub IfType: IFTYPE,
    pub OperStatus: IF_OPER_STATUS,
    pub Ipv6IfIndex: IF_INDEX,
    pub ZoneIndices: [ULONG; 16usize],
    pub FirstPrefix: PIP_ADAPTER_PREFIX_XP,
    pub TransmitLinkSpeed: ULONG64,
    pub ReceiveLinkSpeed: ULONG64,
    pub FirstWinsServerAddress: PIP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub FirstGatewayAddress: PIP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Ipv4Metric: ULONG,
    pub Ipv6Metric: ULONG,
    pub Luid: IF_LUID,
    pub Dhcpv4Server: SOCKET_ADDRESS,
    pub CompartmentId: NET_IF_COMPARTMENT_ID,
    pub NetworkGuid: NET_IF_NETWORK_GUID,
    pub ConnectionType: NET_IF_CONNECTION_TYPE,
    pub TunnelType: TUNNEL_TYPE,
    pub Dhcpv6Server: SOCKET_ADDRESS,
    pub Dhcpv6ClientDuid: [BYTE; 130usize],
    pub Dhcpv6ClientDuidLength: ULONG,
    pub Dhcpv6Iaid: ULONG,
    pub FirstDnsSuffix: PIP_ADAPTER_DNS_SUFFIX,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1 {
    pub Alignment: ULONGLONG,
    pub __bindgen_anon_1: _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1 {
    pub Length: ULONG,
    pub IfIndex: IF_INDEX,
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1>())).Length
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1>())).IfIndex
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(IfIndex)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1>())).Alignment as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_1),
            "::",
            stringify!(Alignment)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2 {
    pub Flags: ULONG,
    pub __bindgen_anon_1: _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
    pub __bindgen_padding_0: u16,
    pub __bindgen_align: [u32; 0usize],
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1)
        )
    );
}
impl _IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2__bindgen_ty_1 {
    #[inline]
    pub fn DdnsEnabled(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_DdnsEnabled(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn RegisterAdapterSuffix(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_RegisterAdapterSuffix(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Dhcpv4Enabled(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Dhcpv4Enabled(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ReceiveOnly(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_ReceiveOnly(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn NoMulticast(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_NoMulticast(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Ipv6OtherStatefulConfig(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Ipv6OtherStatefulConfig(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn NetbiosOverTcpipEnabled(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_NetbiosOverTcpipEnabled(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Ipv4Enabled(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Ipv4Enabled(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Ipv6Enabled(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Ipv6Enabled(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn Ipv6ManagedAddressConfigurationSupported(&self) -> ULONG {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_Ipv6ManagedAddressConfigurationSupported(&mut self, val: ULONG) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        DdnsEnabled: ULONG,
        RegisterAdapterSuffix: ULONG,
        Dhcpv4Enabled: ULONG,
        ReceiveOnly: ULONG,
        NoMulticast: ULONG,
        Ipv6OtherStatefulConfig: ULONG,
        NetbiosOverTcpipEnabled: ULONG,
        Ipv4Enabled: ULONG,
        Ipv6Enabled: ULONG,
        Ipv6ManagedAddressConfigurationSupported: ULONG,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let DdnsEnabled: u32 = unsafe { ::std::mem::transmute(DdnsEnabled) };
            DdnsEnabled as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let RegisterAdapterSuffix: u32 =
                unsafe { ::std::mem::transmute(RegisterAdapterSuffix) };
            RegisterAdapterSuffix as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let Dhcpv4Enabled: u32 = unsafe { ::std::mem::transmute(Dhcpv4Enabled) };
            Dhcpv4Enabled as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let ReceiveOnly: u32 = unsafe { ::std::mem::transmute(ReceiveOnly) };
            ReceiveOnly as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let NoMulticast: u32 = unsafe { ::std::mem::transmute(NoMulticast) };
            NoMulticast as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let Ipv6OtherStatefulConfig: u32 =
                unsafe { ::std::mem::transmute(Ipv6OtherStatefulConfig) };
            Ipv6OtherStatefulConfig as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let NetbiosOverTcpipEnabled: u32 =
                unsafe { ::std::mem::transmute(NetbiosOverTcpipEnabled) };
            NetbiosOverTcpipEnabled as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let Ipv4Enabled: u32 = unsafe { ::std::mem::transmute(Ipv4Enabled) };
            Ipv4Enabled as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let Ipv6Enabled: u32 = unsafe { ::std::mem::transmute(Ipv6Enabled) };
            Ipv6Enabled as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let Ipv6ManagedAddressConfigurationSupported: u32 =
                unsafe { ::std::mem::transmute(Ipv6ManagedAddressConfigurationSupported) };
            Ipv6ManagedAddressConfigurationSupported as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2>())).Flags as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH__bindgen_ty_2),
            "::",
            stringify!(Flags)
        )
    );
}
#[test]
fn bindgen_test_layout__IP_ADAPTER_ADDRESSES_LH() {
    assert_eq!(
        ::std::mem::size_of::<_IP_ADAPTER_ADDRESSES_LH>(),
        448usize,
        concat!("Size of: ", stringify!(_IP_ADAPTER_ADDRESSES_LH))
    );
    assert_eq!(
        ::std::mem::align_of::<_IP_ADAPTER_ADDRESSES_LH>(),
        8usize,
        concat!("Alignment of ", stringify!(_IP_ADAPTER_ADDRESSES_LH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).AdapterName as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(AdapterName)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstUnicastAddress as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstUnicastAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstAnycastAddress as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstAnycastAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstMulticastAddress as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstMulticastAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstDnsServerAddress as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstDnsServerAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).DnsSuffix as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(DnsSuffix)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Description as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Description)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FriendlyName as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FriendlyName)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).PhysicalAddress as *const _
                as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(PhysicalAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).PhysicalAddressLength as *const _
                as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(PhysicalAddressLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Mtu as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Mtu)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).IfType as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(IfType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).OperStatus as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(OperStatus)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Ipv6IfIndex as *const _ as usize
        },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Ipv6IfIndex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).ZoneIndices as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(ZoneIndices)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstPrefix as *const _ as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstPrefix)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).TransmitLinkSpeed as *const _
                as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(TransmitLinkSpeed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).ReceiveLinkSpeed as *const _
                as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(ReceiveLinkSpeed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstWinsServerAddress as *const _
                as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstWinsServerAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstGatewayAddress as *const _
                as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstGatewayAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Ipv4Metric as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Ipv4Metric)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Ipv6Metric as *const _ as usize
        },
        220usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Ipv6Metric)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Luid as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Luid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Dhcpv4Server as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Dhcpv4Server)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).CompartmentId as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(CompartmentId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).NetworkGuid as *const _ as usize
        },
        252usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(NetworkGuid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).ConnectionType as *const _ as usize
        },
        268usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(ConnectionType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).TunnelType as *const _ as usize
        },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(TunnelType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Dhcpv6Server as *const _ as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Dhcpv6Server)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Dhcpv6ClientDuid as *const _
                as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Dhcpv6ClientDuid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Dhcpv6ClientDuidLength as *const _
                as usize
        },
        428usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Dhcpv6ClientDuidLength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).Dhcpv6Iaid as *const _ as usize
        },
        432usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(Dhcpv6Iaid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_IP_ADAPTER_ADDRESSES_LH>())).FirstDnsSuffix as *const _ as usize
        },
        440usize,
        concat!(
            "Offset of field: ",
            stringify!(_IP_ADAPTER_ADDRESSES_LH),
            "::",
            stringify!(FirstDnsSuffix)
        )
    );
}
pub type IP_ADAPTER_ADDRESSES_LH = _IP_ADAPTER_ADDRESSES_LH;
pub type PIP_ADAPTER_ADDRESSES = *mut IP_ADAPTER_ADDRESSES_LH;
extern "C" {
    pub fn GetAdaptersAddresses(
        Family: ULONG,
        Flags: ULONG,
        Reserved: PVOID,
        AdapterAddresses: PIP_ADAPTER_ADDRESSES,
        SizePointer: PULONG,
    ) -> ULONG;
}
