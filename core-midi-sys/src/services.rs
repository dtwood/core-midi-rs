/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

pub type ItemCount = u32;

use core_foundation_sys;
/*

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum Enum_Unnamed_Services1 {
    kMIDIInvalidClient = -10830,
    kMIDIInvalidPort = -10831,
    kMIDIWrongEndpointType = -10832,
    kMIDINoConnection = -10833,
    kMIDIUnknownEndpoint = -10834,
    kMIDIUnknownProperty = -10835,
    kMIDIWrongPropertyType = -10836,
    kMIDINoCurrentSetup = -10837,
    kMIDIMessageSendErr = -10838,
    kMIDIServerStartErr = -10839,
    kMIDISetupFormatErr = -10840,
    kMIDIWrongThread = -10841,
    kMIDIObjectNotFound = -10842,
    kMIDIIDNotUnique = -10843,
    kMIDINotPermitted = -10844,
}
*/
pub type MIDIObjectRef = u32;
pub type MIDIClientRef = MIDIObjectRef;
pub type MIDIPortRef = MIDIObjectRef;
pub type MIDIDeviceRef = MIDIObjectRef;
pub type MIDIEntityRef = MIDIObjectRef;
pub type MIDIEndpointRef = MIDIObjectRef;
pub type MIDITimeStamp = u64;
/*
pub type MIDIObjectType = SInt32;
#[derive(Copy, Clone)]
#[repr(i32)]
pub enum Enum_Unnamed_Services2 {
    kMIDIObjectType_Other = -1,
    kMIDIObjectType_Device = 0,
    kMIDIObjectType_Entity = 1,
    kMIDIObjectType_Source = 2,
    kMIDIObjectType_Destination = 3,
    kMIDIObjectType_ExternalDevice = 16,
    kMIDIObjectType_ExternalEntity = 17,
    kMIDIObjectType_ExternalSource = 18,
    kMIDIObjectType_ExternalDestination = 19,
}
pub type MIDIUniqueID = SInt32;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed_Services3 { kMIDIInvalidUniqueID = 0, }
                                                   */
pub type MIDINotifyProc =
    ::std::option::Option<unsafe extern "C" fn(message:
                                                   *const MIDINotification,
                                               refCon:
                                                   *mut ::std::os::raw::c_void)>;
pub type MIDINotifyBlock =
    ::std::option::Option<unsafe extern "C" fn(message:
                                                   *const MIDINotification)>;
pub type MIDIReadProc =
    ::std::option::Option<unsafe extern "C" fn(pktlist: *const MIDIPacketList,
                                               readProcRefCon:
                                                   *mut ::std::os::raw::c_void,
                                               srcConnRefCon:
                                                   *mut ::std::os::raw::c_void)>;
    /*
pub type MIDIReadBlock =
    ::std::option::Option<unsafe extern "C" fn(pktlist: *const MIDIPacketList,
                                               srcConnRefCon:
                                                   *mut ::std::os::raw::c_void)>;
pub type MIDICompletionProc =
    ::std::option::Option<unsafe extern "C" fn(request:
                                                   *mut MIDISysexSendRequest)>;
*/
#[repr(C)]
pub struct MIDIPacket {
    pub timeStamp: MIDITimeStamp,
    pub length: u16,
    pub data: ::std::os::raw::c_void,
}

#[repr(C)]
pub struct MIDIPacketList {
    pub numPackets: u32,
    pub packet: ::std::os::raw::c_void,
}
/*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDISysexSendRequest {
    pub destination: MIDIEndpointRef,
    pub data: *const Byte,
    pub bytesToSend: UInt32,
    pub complete: Boolean,
    pub reserved: [Byte; 3usize],
    pub completionProc: MIDICompletionProc,
    pub completionRefCon: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for MIDISysexSendRequest {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
*/
pub type MIDINotificationMessageID = i32;
/*
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed_Services4 {
    kMIDIMsgSetupChanged = 1,
    kMIDIMsgObjectAdded = 2,
    kMIDIMsgObjectRemoved = 3,
    kMIDIMsgPropertyChanged = 4,
    kMIDIMsgThruConnectionsChanged = 5,
    kMIDIMsgSerialPortOwnerChanged = 6,
    kMIDIMsgIOError = 7,
}
*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDINotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: u32,
}
/*
impl ::std::default::Default for MIDINotification {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIObjectAddRemoveNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub parent: MIDIObjectRef,
    pub parentType: MIDIObjectType,
    pub child: MIDIObjectRef,
    pub childType: MIDIObjectType,
}
impl ::std::default::Default for MIDIObjectAddRemoveNotification {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIObjectPropertyChangeNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub object: MIDIObjectRef,
    pub objectType: MIDIObjectType,
    pub propertyName: core_foundation_sys::string::CFStringRef,
}
impl ::std::default::Default for MIDIObjectPropertyChangeNotification {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIIOErrorNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub driverDevice: MIDIDeviceRef,
    pub errorCode: core_foundation_sys::base::OSStatus,
}
impl ::std::default::Default for MIDIIOErrorNotification {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "CoreMIDI", kind = "framework")]
extern "C" {
    pub static kMIDIPropertyName: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyManufacturer: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyModel: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyUniqueID: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyDeviceID: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyReceiveChannels: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyTransmitChannels: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyMaxSysExSpeed: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyAdvanceScheduleTimeMuSec: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyIsEmbeddedEntity: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyIsBroadcast: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertySingleRealtimeEntity: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyConnectionUniqueID: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyOffline: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyPrivate: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyDriverOwner: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyFactoryPatchNameFile: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyUserPatchNameFile: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyNameConfiguration: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyImage: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyDriverVersion: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertySupportsGeneralMIDI: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertySupportsMMC: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyCanRoute: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyReceivesClock: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyReceivesMTC: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyReceivesNotes: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyReceivesProgramChanges: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyReceivesBankSelectMSB: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyReceivesBankSelectLSB: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyTransmitsClock: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyTransmitsMTC: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyTransmitsNotes: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyTransmitsProgramChanges: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyTransmitsBankSelectMSB: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyTransmitsBankSelectLSB: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyPanDisruptsStereo: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyIsSampler: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyIsDrumMachine: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyIsMixer: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyIsEffectUnit: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyMaxReceiveChannels: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyMaxTransmitChannels: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyDriverDeviceEditorApp: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertySupportsShowControl: core_foundation_sys::string::CFStringRef;
    pub static kMIDIPropertyDisplayName: core_foundation_sys::string::CFStringRef;
}
*/
#[link(name = "CoreMIDI", kind = "framework")]
extern "C" {
    pub fn MIDIClientCreate(name: core_foundation_sys::string::CFStringRef, notifyProc: MIDINotifyProc,
                            notifyRefCon: *mut ::std::os::raw::c_void,
                            outClient: *mut MIDIClientRef) -> core_foundation_sys::base::OSStatus;
    /*
    pub fn MIDIClientCreateWithBlock(name: core_foundation_sys::string::CFStringRef,
                                     outClient: *mut MIDIClientRef,
                                     notifyBlock: MIDINotifyBlock)
     -> core_foundation_sys::base::OSStatus;
     */
    pub fn MIDIClientDispose(client: MIDIClientRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIInputPortCreate(client: MIDIClientRef, portName: core_foundation_sys::string::CFStringRef,
                               readProc: MIDIReadProc,
                               refCon: *mut ::std::os::raw::c_void,
                               outPort: *mut MIDIPortRef) -> core_foundation_sys::base::OSStatus;
    /*
    pub fn MIDIInputPortCreateWithBlock(client: MIDIClientRef,
                                        portName: core_foundation_sys::string::CFStringRef,
                                        outPort: *mut MIDIPortRef,
                                        readBlock: MIDIReadBlock) -> core_foundation_sys::base::OSStatus;
    */
    pub fn MIDIOutputPortCreate(client: MIDIClientRef, portName: core_foundation_sys::string::CFStringRef,
                                outPort: *mut MIDIPortRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIPortDispose(port: MIDIPortRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIPortConnectSource(port: MIDIPortRef, source: MIDIEndpointRef,
                                 connRefCon: *mut ::std::os::raw::c_void)
     -> core_foundation_sys::base::OSStatus;
    pub fn MIDIPortDisconnectSource(port: MIDIPortRef,
                                    source: MIDIEndpointRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIGetNumberOfDevices() -> ItemCount;
    pub fn MIDIGetDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
    pub fn MIDIDeviceGetNumberOfEntities(device: MIDIDeviceRef) -> ItemCount;
    pub fn MIDIDeviceGetEntity(device: MIDIDeviceRef, entityIndex0: ItemCount)
     -> MIDIEntityRef;
    pub fn MIDIEntityGetNumberOfSources(entity: MIDIEntityRef) -> ItemCount;
    pub fn MIDIEntityGetSource(entity: MIDIEntityRef, sourceIndex0: ItemCount)
     -> MIDIEndpointRef;
    pub fn MIDIEntityGetNumberOfDestinations(entity: MIDIEntityRef)
     -> ItemCount;
    pub fn MIDIEntityGetDestination(entity: MIDIEntityRef,
                                    destIndex0: ItemCount) -> MIDIEndpointRef;
    pub fn MIDIEntityGetDevice(inEntity: MIDIEntityRef,
                               outDevice: *mut MIDIDeviceRef) -> core_foundation_sys::base::OSStatus;
    /*
    pub fn MIDIGetNumberOfSources() -> ItemCount;
    pub fn MIDIGetSource(sourceIndex0: ItemCount) -> MIDIEndpointRef;
    pub fn MIDIGetNumberOfDestinations() -> ItemCount;
    pub fn MIDIGetDestination(destIndex0: ItemCount) -> MIDIEndpointRef;
    pub fn MIDIEndpointGetEntity(inEndpoint: MIDIEndpointRef,
                                 outEntity: *mut MIDIEntityRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIDestinationCreate(client: MIDIClientRef, name: core_foundation_sys::string::CFStringRef,
                                 readProc: MIDIReadProc,
                                 refCon: *mut ::std::os::raw::c_void,
                                 outDest: *mut MIDIEndpointRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIDestinationCreateWithBlock(client: MIDIClientRef,
                                          name: core_foundation_sys::string::CFStringRef,
                                          outDest: *mut MIDIEndpointRef,
                                          readBlock: MIDIReadBlock)
     -> core_foundation_sys::base::OSStatus;
     */
    pub fn MIDISourceCreate(client: MIDIClientRef, name: core_foundation_sys::string::CFStringRef,
                            outSrc: *mut MIDIEndpointRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIEndpointDispose(endpt: MIDIEndpointRef) -> core_foundation_sys::base::OSStatus;
}
/*
    pub fn MIDIGetNumberOfExternalDevices() -> ItemCount;
    pub fn MIDIGetExternalDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
    pub fn MIDIObjectGetIntegerProperty(obj: MIDIObjectRef,
                                        propertyID: core_foundation_sys::string::CFStringRef,
                                        outValue: *mut SInt32) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectSetIntegerProperty(obj: MIDIObjectRef,
                                        propertyID: core_foundation_sys::string::CFStringRef,
                                        value: SInt32) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectGetStringProperty(obj: MIDIObjectRef,
                                       propertyID: core_foundation_sys::string::CFStringRef,
                                       str: *mut core_foundation_sys::string::CFStringRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectSetStringProperty(obj: MIDIObjectRef,
                                       propertyID: core_foundation_sys::string::CFStringRef,
                                       str: core_foundation_sys::string::CFStringRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectGetDataProperty(obj: MIDIObjectRef,
                                     propertyID: core_foundation_sys::string::CFStringRef,
                                     outData: *mut CFDataRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectSetDataProperty(obj: MIDIObjectRef,
                                     propertyID: core_foundation_sys::string::CFStringRef, data: CFDataRef)
     -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectGetDictionaryProperty(obj: MIDIObjectRef,
                                           propertyID: core_foundation_sys::string::CFStringRef,
                                           outDict: *mut CFDictionaryRef)
     -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectSetDictionaryProperty(obj: MIDIObjectRef,
                                           propertyID: core_foundation_sys::string::CFStringRef,
                                           dict: CFDictionaryRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectGetProperties(obj: MIDIObjectRef,
                                   outProperties: *mut CFPropertyListRef,
                                   deep: Boolean) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectRemoveProperty(obj: MIDIObjectRef,
                                    propertyID: core_foundation_sys::string::CFStringRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIObjectFindByUniqueID(inUniqueID: MIDIUniqueID,
                                    outObject: *mut MIDIObjectRef,
                                    outObjectType: *mut MIDIObjectType)
     -> core_foundation_sys::base::OSStatus;
    pub fn MIDISend(port: MIDIPortRef, dest: MIDIEndpointRef,
                    pktlist: *const MIDIPacketList) -> core_foundation_sys::base::OSStatus;
    pub fn MIDISendSysex(request: *mut MIDISysexSendRequest) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIReceived(src: MIDIEndpointRef, pktlist: *const MIDIPacketList)
     -> core_foundation_sys::base::OSStatus;
    pub fn MIDIFlushOutput(dest: MIDIEndpointRef) -> core_foundation_sys::base::OSStatus;
    pub fn MIDIRestart() -> core_foundation_sys::base::OSStatus;
    pub fn MIDIPacketListInit(pktlist: *mut MIDIPacketList)
     -> *mut MIDIPacket;
    pub fn MIDIPacketListAdd(pktlist: *mut MIDIPacketList,
                             listSize: ByteCount, curPacket: *mut MIDIPacket,
                             time: MIDITimeStamp, nData: ByteCount,
                             data: *const Byte) -> *mut MIDIPacket;
}

*/
