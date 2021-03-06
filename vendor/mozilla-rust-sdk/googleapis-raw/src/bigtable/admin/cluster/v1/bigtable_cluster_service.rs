// This file is generated by rust-protobuf 2.7.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/bigtable/admin/cluster/v1/bigtable_cluster_service.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_7_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n?google/bigtable/admin/cluster/v1/bigtable_cluster_service.proto\x12\
    \x20google.bigtable.admin.cluster.v1\x1a\x1cgoogle/api/annotations.proto\
    \x1a<google/bigtable/admin/cluster/v1/bigtable_cluster_data.proto\x1aHgo\
    ogle/bigtable/admin/cluster/v1/bigtable_cluster_service_messages.proto\
    \x1a#google/longrunning/operations.proto\x1a\x1bgoogle/protobuf/empty.pr\
    oto2\x89\t\n\x16BigtableClusterService\x12\x99\x01\n\tListZones\x122.goo\
    gle.bigtable.admin.cluster.v1.ListZonesRequest\x1a3.google.bigtable.admi\
    n.cluster.v1.ListZonesResponse\"#\x82\xd3\xe4\x93\x02\x1d\x12\x1b/v1/{na\
    me=projects/*}/zones\x12\x9e\x01\n\nGetCluster\x123.google.bigtable.admi\
    n.cluster.v1.GetClusterRequest\x1a).google.bigtable.admin.cluster.v1.Clu\
    ster\"0\x82\xd3\xe4\x93\x02*\x12(/v1/{name=projects/*/zones/*/clusters/*\
    }\x12\xb0\x01\n\x0cListClusters\x125.google.bigtable.admin.cluster.v1.Li\
    stClustersRequest\x1a6.google.bigtable.admin.cluster.v1.ListClustersResp\
    onse\"1\x82\xd3\xe4\x93\x02+\x12)/v1/{name=projects/*}/aggregated/cluste\
    rs\x12\xa5\x01\n\rCreateCluster\x126.google.bigtable.admin.cluster.v1.Cr\
    eateClusterRequest\x1a).google.bigtable.admin.cluster.v1.Cluster\"1\x82\
    \xd3\xe4\x93\x02+\"&/v1/{name=projects/*/zones/*}/clusters:\x01*\x12\x9a\
    \x01\n\rUpdateCluster\x12).google.bigtable.admin.cluster.v1.Cluster\x1a)\
    .google.bigtable.admin.cluster.v1.Cluster\"3\x82\xd3\xe4\x93\x02-\x1a(/v\
    1/{name=projects/*/zones/*/clusters/*}:\x01*\x12\x91\x01\n\rDeleteCluste\
    r\x126.google.bigtable.admin.cluster.v1.DeleteClusterRequest\x1a\x16.goo\
    gle.protobuf.Empty\"0\x82\xd3\xe4\x93\x02**(/v1/{name=projects/*/zones/*\
    /clusters/*}\x12\xa5\x01\n\x0fUndeleteCluster\x128.google.bigtable.admin\
    .cluster.v1.UndeleteClusterRequest\x1a\x1d.google.longrunning.Operation\
    \"9\x82\xd3\xe4\x93\x023\"1/v1/{name=projects/*/zones/*/clusters/*}:unde\
    leteB\x8f\x01\n$com.google.bigtable.admin.cluster.v1B\x1cBigtableCluster\
    ServicesProtoP\x01ZGgoogle.golang.org/genproto/googleapis/bigtable/admin\
    /cluster/v1;clusterJ\x9d,\n\x07\x12\x05\x0e\0\x81\x01\x01\n\xbd\x04\n\
    \x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202017\x20Google\x20In\
    c.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202\
    .0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20fil\
    e\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20m\
    ay\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0)\n\t\n\x02\x03\0\x12\x03\
    \x12\0&\n\t\n\x02\x03\x01\x12\x03\x13\0F\n\t\n\x02\x03\x02\x12\x03\x14\0\
    R\n\t\n\x02\x03\x03\x12\x03\x15\0-\n\t\n\x02\x03\x04\x12\x03\x16\0%\n\
    \x08\n\x01\x08\x12\x03\x18\0^\n\t\n\x02\x08\x0b\x12\x03\x18\0^\n\x08\n\
    \x01\x08\x12\x03\x19\0\"\n\t\n\x02\x08\n\x12\x03\x19\0\"\n\x08\n\x01\x08\
    \x12\x03\x1a\0=\n\t\n\x02\x08\x08\x12\x03\x1a\0=\n\x08\n\x01\x08\x12\x03\
    \x1b\0=\n\t\n\x02\x08\x01\x12\x03\x1b\0=\nC\n\x02\x06\0\x12\x05\x1f\0\
    \x81\x01\x01\x1a6\x20Service\x20for\x20managing\x20zonal\x20Cloud\x20Big\
    table\x20resources.\n\n\n\n\x03\x06\0\x01\x12\x03\x1f\x08\x1e\n@\n\x04\
    \x06\0\x02\0\x12\x04!\x02#\x03\x1a2\x20Lists\x20the\x20supported\x20zone\
    s\x20for\x20the\x20given\x20project.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\
    \x03!\x06\x0f\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03!\x10\x20\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x03!+<\n\x0c\n\x05\x06\0\x02\0\x04\x12\x03\"\x04F\n\
    \x10\n\t\x06\0\x02\0\x04\xb0\xca\xbc\"\x12\x03\"\x04F\n<\n\x04\x06\0\x02\
    \x01\x12\x04&\x02(\x03\x1a.\x20Gets\x20information\x20about\x20a\x20part\
    icular\x20cluster.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03&\x06\x10\n\
    \x0c\n\x05\x06\0\x02\x01\x02\x12\x03&\x11\"\n\x0c\n\x05\x06\0\x02\x01\
    \x03\x12\x03&-4\n\x0c\n\x05\x06\0\x02\x01\x04\x12\x03'\x04S\n\x10\n\t\
    \x06\0\x02\x01\x04\xb0\xca\xbc\"\x12\x03'\x04S\n\x84\x01\n\x04\x06\0\x02\
    \x02\x12\x04,\x02.\x03\x1av\x20Lists\x20all\x20clusters\x20in\x20the\x20\
    given\x20project,\x20along\x20with\x20any\x20zones\x20for\x20which\n\x20\
    cluster\x20information\x20could\x20not\x20be\x20retrieved.\n\n\x0c\n\x05\
    \x06\0\x02\x02\x01\x12\x03,\x06\x12\n\x0c\n\x05\x06\0\x02\x02\x02\x12\
    \x03,\x13&\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03,1E\n\x0c\n\x05\x06\0\
    \x02\x02\x04\x12\x03-\x04T\n\x10\n\t\x06\0\x02\x02\x04\xb0\xca\xbc\"\x12\
    \x03-\x04T\n\xec\x08\n\x04\x06\0\x02\x03\x12\x04B\x02D\x03\x1a\xdd\x08\
    \x20Creates\x20a\x20cluster\x20and\x20begins\x20preparing\x20it\x20to\
    \x20begin\x20serving.\x20The\x20returned\n\x20cluster\x20embeds\x20as\
    \x20its\x20\"current_operation\"\x20a\x20long-running\x20operation\x20wh\
    ich\n\x20can\x20be\x20used\x20to\x20track\x20the\x20progress\x20of\x20tu\
    rning\x20up\x20the\x20new\x20cluster.\n\x20Immediately\x20upon\x20comple\
    tion\x20of\x20this\x20request:\n\x20\x20*\x20The\x20cluster\x20will\x20b\
    e\x20readable\x20via\x20the\x20API,\x20with\x20all\x20requested\x20attri\
    butes\n\x20\x20\x20\x20but\x20no\x20allocated\x20resources.\n\x20Until\
    \x20completion\x20of\x20the\x20embedded\x20operation:\n\x20\x20*\x20Canc\
    elling\x20the\x20operation\x20will\x20render\x20the\x20cluster\x20immedi\
    ately\x20unreadable\n\x20\x20\x20\x20via\x20the\x20API.\n\x20\x20*\x20Al\
    l\x20other\x20attempts\x20to\x20modify\x20or\x20delete\x20the\x20cluster\
    \x20will\x20be\x20rejected.\n\x20Upon\x20completion\x20of\x20the\x20embe\
    dded\x20operation:\n\x20\x20*\x20Billing\x20for\x20all\x20successfully-a\
    llocated\x20resources\x20will\x20begin\x20(some\x20types\n\x20\x20\x20\
    \x20may\x20have\x20lower\x20than\x20the\x20requested\x20levels).\n\x20\
    \x20*\x20New\x20tables\x20can\x20be\x20created\x20in\x20the\x20cluster.\
    \n\x20\x20*\x20The\x20cluster's\x20allocated\x20resource\x20levels\x20wi\
    ll\x20be\x20readable\x20via\x20the\x20API.\n\x20The\x20embedded\x20opera\
    tion's\x20\"metadata\"\x20field\x20type\x20is\n\x20[CreateClusterMetadat\
    a][google.bigtable.admin.cluster.v1.CreateClusterMetadata]\x20The\x20emb\
    edded\x20operation's\x20\"response\"\x20field\x20type\x20is\n\x20[Cluste\
    r][google.bigtable.admin.cluster.v1.Cluster],\x20if\x20successful.\n\n\
    \x0c\n\x05\x06\0\x02\x03\x01\x12\x03B\x06\x13\n\x0c\n\x05\x06\0\x02\x03\
    \x02\x12\x03B\x14(\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03B3:\n\x0c\n\x05\
    \x06\0\x02\x03\x04\x12\x03C\x04\\\n\x10\n\t\x06\0\x02\x03\x04\xb0\xca\
    \xbc\"\x12\x03C\x04\\\n\xa7\x0b\n\x04\x06\0\x02\x04\x12\x04]\x02_\x03\
    \x1a\x98\x0b\x20Updates\x20a\x20cluster,\x20and\x20begins\x20allocating\
    \x20or\x20releasing\x20resources\x20as\n\x20requested.\x20The\x20returne\
    d\x20cluster\x20embeds\x20as\x20its\x20\"current_operation\"\x20a\n\x20l\
    ong-running\x20operation\x20which\x20can\x20be\x20used\x20to\x20track\
    \x20the\x20progress\x20of\x20updating\n\x20the\x20cluster.\n\x20Immediat\
    ely\x20upon\x20completion\x20of\x20this\x20request:\n\x20\x20*\x20For\
    \x20resource\x20types\x20where\x20a\x20decrease\x20in\x20the\x20cluster'\
    s\x20allocation\x20has\x20been\n\x20\x20\x20\x20requested,\x20billing\
    \x20will\x20be\x20based\x20on\x20the\x20newly-requested\x20level.\n\x20U\
    ntil\x20completion\x20of\x20the\x20embedded\x20operation:\n\x20\x20*\x20\
    Cancelling\x20the\x20operation\x20will\x20set\x20its\x20metadata's\x20\"\
    cancelled_at_time\",\n\x20\x20\x20\x20and\x20begin\x20restoring\x20resou\
    rces\x20to\x20their\x20pre-request\x20values.\x20The\x20operation\n\x20\
    \x20\x20\x20is\x20guaranteed\x20to\x20succeed\x20at\x20undoing\x20all\
    \x20resource\x20changes,\x20after\x20which\n\x20\x20\x20\x20point\x20it\
    \x20will\x20terminate\x20with\x20a\x20CANCELLED\x20status.\n\x20\x20*\
    \x20All\x20other\x20attempts\x20to\x20modify\x20or\x20delete\x20the\x20c\
    luster\x20will\x20be\x20rejected.\n\x20\x20*\x20Reading\x20the\x20cluste\
    r\x20via\x20the\x20API\x20will\x20continue\x20to\x20give\x20the\x20pre-r\
    equest\n\x20\x20\x20\x20resource\x20levels.\n\x20Upon\x20completion\x20o\
    f\x20the\x20embedded\x20operation:\n\x20\x20*\x20Billing\x20will\x20begi\
    n\x20for\x20all\x20successfully-allocated\x20resources\x20(some\x20types\
    \n\x20\x20\x20\x20may\x20have\x20lower\x20than\x20the\x20requested\x20le\
    vels).\n\x20\x20*\x20All\x20newly-reserved\x20resources\x20will\x20be\
    \x20available\x20for\x20serving\x20the\x20cluster's\n\x20\x20\x20\x20tab\
    les.\n\x20\x20*\x20The\x20cluster's\x20new\x20resource\x20levels\x20will\
    \x20be\x20readable\x20via\x20the\x20API.\n\x20[UpdateClusterMetadata][go\
    ogle.bigtable.admin.cluster.v1.UpdateClusterMetadata]\x20The\x20embedded\
    \x20operation's\x20\"response\"\x20field\x20type\x20is\n\x20[Cluster][go\
    ogle.bigtable.admin.cluster.v1.Cluster],\x20if\x20successful.\n\n\x0c\n\
    \x05\x06\0\x02\x04\x01\x12\x03]\x06\x13\n\x0c\n\x05\x06\0\x02\x04\x02\
    \x12\x03]\x14\x1b\n\x0c\n\x05\x06\0\x02\x04\x03\x12\x03]&-\n\x0c\n\x05\
    \x06\0\x02\x04\x04\x12\x03^\x04]\n\x10\n\t\x06\0\x02\x04\x04\xb0\xca\xbc\
    \"\x12\x03^\x04]\n\xc0\x05\n\x04\x06\0\x02\x05\x12\x04m\x02o\x03\x1a\xb1\
    \x05\x20Marks\x20a\x20cluster\x20and\x20all\x20of\x20its\x20tables\x20fo\
    r\x20permanent\x20deletion\x20in\x207\x20days.\n\x20Immediately\x20upon\
    \x20completion\x20of\x20the\x20request:\n\x20\x20*\x20Billing\x20will\
    \x20cease\x20for\x20all\x20of\x20the\x20cluster's\x20reserved\x20resourc\
    es.\n\x20\x20*\x20The\x20cluster's\x20\"delete_time\"\x20field\x20will\
    \x20be\x20set\x207\x20days\x20in\x20the\x20future.\n\x20Soon\x20afterwar\
    d:\n\x20\x20*\x20All\x20tables\x20within\x20the\x20cluster\x20will\x20be\
    come\x20unavailable.\n\x20Prior\x20to\x20the\x20cluster's\x20\"delete_ti\
    me\":\n\x20\x20*\x20The\x20cluster\x20can\x20be\x20recovered\x20with\x20\
    a\x20call\x20to\x20UndeleteCluster.\n\x20\x20*\x20All\x20other\x20attemp\
    ts\x20to\x20modify\x20or\x20delete\x20the\x20cluster\x20will\x20be\x20re\
    jected.\n\x20At\x20the\x20cluster's\x20\"delete_time\":\n\x20\x20*\x20Th\
    e\x20cluster\x20and\x20*all\x20of\x20its\x20tables*\x20will\x20immediate\
    ly\x20and\x20irrevocably\n\x20\x20\x20\x20disappear\x20from\x20the\x20AP\
    I,\x20and\x20their\x20data\x20will\x20be\x20permanently\x20deleted.\n\n\
    \x0c\n\x05\x06\0\x02\x05\x01\x12\x03m\x06\x13\n\x0c\n\x05\x06\0\x02\x05\
    \x02\x12\x03m\x14(\n\x0c\n\x05\x06\0\x02\x05\x03\x12\x03m3H\n\x0c\n\x05\
    \x06\0\x02\x05\x04\x12\x03n\x04V\n\x10\n\t\x06\0\x02\x05\x04\xb0\xca\xbc\
    \"\x12\x03n\x04V\n\x82\x06\n\x04\x06\0\x02\x06\x12\x05~\x02\x80\x01\x03\
    \x1a\xf2\x05\x20Cancels\x20the\x20scheduled\x20deletion\x20of\x20an\x20c\
    luster\x20and\x20begins\x20preparing\x20it\x20to\n\x20resume\x20serving.\
    \x20The\x20returned\x20operation\x20will\x20also\x20be\x20embedded\x20as\
    \x20the\n\x20cluster's\x20\"current_operation\".\n\x20Immediately\x20upo\
    n\x20completion\x20of\x20this\x20request:\n\x20\x20*\x20The\x20cluster's\
    \x20\"delete_time\"\x20field\x20will\x20be\x20unset,\x20protecting\x20it\
    \x20from\n\x20\x20\x20\x20automatic\x20deletion.\n\x20Until\x20completio\
    n\x20of\x20the\x20returned\x20operation:\n\x20\x20*\x20The\x20operation\
    \x20cannot\x20be\x20cancelled.\n\x20Upon\x20completion\x20of\x20the\x20r\
    eturned\x20operation:\n\x20\x20*\x20Billing\x20for\x20the\x20cluster's\
    \x20resources\x20will\x20resume.\n\x20\x20*\x20All\x20tables\x20within\
    \x20the\x20cluster\x20will\x20be\x20available.\n\x20[UndeleteClusterMeta\
    data][google.bigtable.admin.cluster.v1.UndeleteClusterMetadata]\x20The\
    \x20embedded\x20operation's\x20\"response\"\x20field\x20type\x20is\n\x20\
    [Cluster][google.bigtable.admin.cluster.v1.Cluster],\x20if\x20successful\
    .\n\n\x0c\n\x05\x06\0\x02\x06\x01\x12\x03~\x06\x15\n\x0c\n\x05\x06\0\x02\
    \x06\x02\x12\x03~\x16,\n\x0c\n\x05\x06\0\x02\x06\x03\x12\x03~7S\n\x0c\n\
    \x05\x06\0\x02\x06\x04\x12\x03\x7f\x04f\n\x10\n\t\x06\0\x02\x06\x04\xb0\
    \xca\xbc\"\x12\x03\x7f\x04fb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
