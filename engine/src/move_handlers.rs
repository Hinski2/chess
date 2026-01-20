use crate::{Color, Piece, PieceColor};
use crate::board::Board;
use crate::piece_move::{MoveFlag, PieceMove};

const CASTLING_RIGHTS_UPDATE: [u8; 64] = [
    0b1101, 0b1111, 0b1111, 0b1111, 0b1100, 0b1111, 0b1111, 0b1110,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b0111, 0b1111, 0b1111, 0b1111, 0b0011, 0b1111, 0b1111, 0b1011,
];

const PIECE_COLOR_HSH: [[u64; 64]; 13] = [
	[ 0x9a43059036ff1782u64, 0x8f73b3ab27629363u64, 0x43f25d05dcea7cb5u64, 0xe9984c2a16274e3eu64, 0xf62f60b0b70ab044u64, 0x2b3a432ebb7dbf57u64, 0xd742bca2b8328cfcu64, 0xe2d3bef1e2c8ac97u64, 0x03f3ae2964c7f196u64, 0xe4d99175e3d878e0u64, 0xfc25f6ca3094754eu64, 0xb72cbc3dbc6971ccu64, 0x6d0befffe65e09cdu64, 0xa8fee4dbc72b0b44u64, 0x181ad768966a5df5u64, 0x5407f0d15e5d44cfu64, 0xaf4c044ce0184375u64, 0xd78e5ea65e1f23fdu64, 0x7d76765fe28794a6u64, 0x0898076bc31490bbu64, 0x535c3f79e92a6cf0u64, 0xa08c66da3000a4b0u64, 0x4d0006e538fc27deu64, 0x8f78bf065fb60fecu64, 0xe938a345d5bda125u64, 0xb83c4124edf63abcu64, 0x340ea5e013baa949u64, 0xd918dfc0483626dfu64, 0xc7420a47e762ec52u64, 0x2910857638506d85u64, 0xe0c2b0c816b598d7u64, 0x6c9c859c895dfb2eu64, 0x461d0f46d7642fb1u64, 0x8555067a7393442bu64, 0x71c269d1ce5a11b9u64, 0x3d782b33e8df3df5u64, 0x78c0ce232d551e2eu64, 0x6ec32ae07328e2edu64, 0x56f3984405fd8bb1u64, 0xa3334f3e063bb222u64, 0x7d6971c99a4215ccu64, 0x70f2407b10d6f6f8u64, 0xe123210269dbaa60u64, 0x0731b335090473b1u64, 0xfa0e6314886ddc93u64, 0xe526ffd4b5a1305eu64, 0xedd0aa1972cecc97u64, 0x0209f3ff51c55c7fu64, 0x0dd4dbe70ab3a48fu64, 0x24af0c0cded15be7u64, 0x6a3e73794f1c7842u64, 0x21a2337354d9e8b0u64, 0x2e5ddd11053f9887u64, 0x6d1a130396ee6e14u64, 0xdedc39f0d59cfc9bu64, 0x7acf9fb174a8418au64, 0x522ae883553d479cu64, 0x0c0d4b12d03448c0u64, 0xb08393f43e082e4cu64, 0x8705f350155683bbu64, 0xee8b4c134ec26d45u64, 0x68a9c87cda11af27u64, 0xcff421c390acfff3u64, 0x37f7b43593a3a5b7u64, ],
	[ 0x25c7f6c18d1e6073u64, 0x16735c3c4c1f0f99u64, 0xdbf134f955541baau64, 0x569190b69927a881u64, 0x3fdfbf01e35c1fdbu64, 0x9d2c3aacb01267b3u64, 0x166804c7980a67adu64, 0xe13fffece6722270u64, 0x0fe22871c0de163du64, 0xdcddd302c8d6af65u64, 0x82b969390a7a5e1au64, 0xa021901c45f9539fu64, 0x00a85b0392da6c03u64, 0x5fa3f47b1ae763feu64, 0x0b5ceac14d33724eu64, 0xa725ab9ceba4317eu64, 0xf5e7e237ffce30bcu64, 0x2039728c47476777u64, 0x173c7f750983615fu64, 0x1f4b30ddc741d038u64, 0x281f8951411c7496u64, 0x3ff6c91b0d337651u64, 0x9fbcc95053d50b00u64, 0xdf3404e8d6ac210bu64, 0x5d17367e388d685au64, 0xe25644903daa01cdu64, 0x8354589240260316u64, 0x00600c9ab72d52f1u64, 0x93be3080789db49au64, 0x1a004c1a9713d1edu64, 0xfe1835fab1cbc2a1u64, 0x7d0cc9ea09f922c6u64, 0xaf73c43d456455dau64, 0x375e7ccf96ca0359u64, 0x44ea290bca8e38e3u64, 0x4314efffcef9d279u64, 0x4206b128a422c6c2u64, 0x44a84284be88e3a3u64, 0x56a7915f7079b77eu64, 0x456684cfd7ba3a21u64, 0x233e6f29faf6194cu64, 0x77a4c36787577135u64, 0x3a5c034a36c4f688u64, 0xdc748405f4bd6559u64, 0x87eb7cb9363a661du64, 0xdd67ae168f7bf772u64, 0xe06089e1cc2192d8u64, 0xafa3ea2b951b467fu64, 0x00d4e51f12e99cf1u64, 0xf24fb5dcdaccc841u64, 0x053d6eef0b4aa60du64, 0x18706e2c0577c64au64, 0x4541558b9e942d29u64, 0x46f2440730495dfdu64, 0x16d0ca4bedc011c0u64, 0x6c8490e0ebc0c961u64, 0xb131362b822e59e7u64, 0x678e10ed3dcddb35u64, 0x39a0574b69e05950u64, 0x91984364a41a64a9u64, 0xf9fa55e3d40d3433u64, 0xa58bac0f7b33e84cu64, 0x4d095c880d378a76u64, 0xdb6300c9d14b660bu64, ],
	[ 0xbf419318882ef671u64, 0x8666d2834b409fa3u64, 0xb0f0e14d0e0b97b5u64, 0x74deb887ba3fdf68u64, 0xd9b9c86aaf9ce336u64, 0x27886784eabbd9cdu64, 0x1946415b6fe6917du64, 0xc585f9a381d26e22u64, 0xcb538518a64a1237u64, 0x43ff52bffb5f5154u64, 0xc664c6c56f91e106u64, 0x7862da7f60eccda8u64, 0x1c20ac57666a0617u64, 0xd2e0c53dd8477b82u64, 0x86c6e9fb96e679b4u64, 0xd3221a51906154a9u64, 0x94d9e47cd16aef82u64, 0xade59ee17ee30adcu64, 0x21e5d9379d7d51a0u64, 0x2615424c8338c1a7u64, 0x37fbf8a3725e0263u64, 0x05b0470340df4cb8u64, 0x75b6bdf484d3bc89u64, 0xfb10810492307122u64, 0xf6ec0d4e22c336d2u64, 0xcc8ee0050e545c60u64, 0x887b65f5a583b53du64, 0xad27675ef5435dafu64, 0x0829319db917447eu64, 0x331762d855e3389eu64, 0x1ff4a45aaa3b9cd8u64, 0xf4ca815dcc2c0a80u64, 0x8d0850c143160486u64, 0xb166833782646ea0u64, 0x095181c2154bcaf7u64, 0x937ab0325095bbe5u64, 0xaf992618faaed4aeu64, 0x122538e728eb3485u64, 0xd7baa8db61ac6b40u64, 0x105a0869b557684fu64, 0xa1c0c65861c1ef7eu64, 0x49d8d6d52383b6a6u64, 0x2358382385ed81f1u64, 0x30bed9f3619996bau64, 0xfc64a1291eacc347u64, 0x032a5c4924a07dc8u64, 0x79f974df0b06af85u64, 0xd119450239c96969u64, 0x4f9cf408fd2a7842u64, 0x8c67c499617b7f9eu64, 0xf79eeb5416c1142au64, 0xc63dce1d9d8d8d74u64, 0xfa6351861c5b7f81u64, 0xbf19671d032808cdu64, 0x1b128f8357aef284u64, 0xb8e7253d32268085u64, 0xa5b154a82500284du64, 0xf4d82aba8d3a8f1eu64, 0x5238ea7fc4380ef3u64, 0x97c6e59c028baed1u64, 0xc6aba8fe1f462a6au64, 0x5dcaef0ee4a08f27u64, 0x1bf29eaaa769af16u64, 0x5bc190fef3b9c41fu64, ],
	[ 0x07c840b21fd8ec82u64, 0x0953597961bb07d5u64, 0x043191f745911f8du64, 0xa99bc3a3c87322c0u64, 0xc8d3a2487e945b58u64, 0xc35cbdd9f03dd860u64, 0xbfa48dc72011c9d8u64, 0xdc78a5dd3735f23au64, 0xaec4f9a01a7946e1u64, 0xea9be93714b5ea2eu64, 0x9cf61cd29fe2841du64, 0x6873933e254df364u64, 0x2b1d325b0fa40d6au64, 0xe441e8223a094d73u64, 0x998808f387250d65u64, 0x4853ee603f38baf5u64, 0x2c3cc319690b0dd6u64, 0x7e5354c71744cf1eu64, 0x4a9b377d2401ad04u64, 0x7bec0579ebc5ded9u64, 0x6c07261791c15823u64, 0xfd805bda6430062eu64, 0x03ccc1ee20814516u64, 0x36c5a3c5b478e01du64, 0xd1681a453c19ab60u64, 0xb351ba5557dc0d76u64, 0xb1163d22a7f81815u64, 0x4c44edcfefb89907u64, 0xe87ec3cd45508e18u64, 0x79da9f75a5baf96du64, 0x86ed3ecce6e1ef8bu64, 0xdfdc6642d8d9356bu64, 0xb4cc6668dbe393d4u64, 0xb69871d8335da0bbu64, 0x7b5c226dc6a13eb3u64, 0xc5f1633131ba243du64, 0x2bd59d277fb28c4bu64, 0x21745fc6f8bda83bu64, 0xa9dda300d7c3478cu64, 0x69cd76a58874c602u64, 0xc8d980fa4976f149u64, 0x39efefd3481d5ec6u64, 0x39fffe7d1fb9da10u64, 0x0c8c9a98aa4490e9u64, 0xf8009b8c290022b5u64, 0xb71c15b2c83c76a9u64, 0xbb800af894bd4f41u64, 0x8f524a527c77c6ebu64, 0x31b8cfb2b0f87609u64, 0x9e78caf667908ff3u64, 0xe83765eb7d4e5600u64, 0x5414fbfa049b6c5du64, 0xff78e06c54771020u64, 0x2c35c42a15c60213u64, 0x0e709e7fcfd58cd8u64, 0x0d504ea248aa4998u64, 0x14b121e41a0041f6u64, 0xeb5b959fba84dbe1u64, 0xa00d26cdcff0197bu64, 0xf051a80e808754fau64, 0x06918b3c66ddd962u64, 0x575fb8ff64a72eb5u64, 0x3195b60cce10c9bau64, 0xdd6ad5e70c78aca0u64, ],
	[ 0xbec453be84bc47edu64, 0x8fe164ee56072eb0u64, 0x7fd8605c07bc0197u64, 0x8f4a71230b2d625au64, 0x14867674688f756bu64, 0xb9b8e44fd9dec82bu64, 0x1bf9e1d1d6cb080fu64, 0x05fea752e688f1f6u64, 0x274112952b39daa1u64, 0x95471508772da439u64, 0xab3a585d9bac809au64, 0x30f5da0163a29152u64, 0xd5f368d0379ea1f4u64, 0x1f68c3d010bf60e4u64, 0x8a875fcf9d36d108u64, 0xebdcf3c37e39d27bu64, 0xc39e5f4675a752e5u64, 0xfac1fd6dad79c4e7u64, 0xdf54824b01d2addau64, 0x71850692f05ae5c3u64, 0xe65101aa3c7b4638u64, 0xd0c92e1b88f04c59u64, 0x67d18468798d3bd6u64, 0x36ed8c8567426df8u64, 0x4c5004a91af34234u64, 0x4295e06804ecab41u64, 0x6577b6e7461d0539u64, 0x3fcdac356aa17e86u64, 0xd671fffc45c107aau64, 0xdf3dbd7a132a64d7u64, 0xe4c8f75dd6d8f411u64, 0xa12391e0a44ddfe0u64, 0x0a528ad7dad75ae6u64, 0x5f22f579db45c5ebu64, 0x9b4d4f114426daf7u64, 0x13c622c1e8e2ed12u64, 0x102329a586e66653u64, 0x540f04ea59702203u64, 0x354d26be59413f19u64, 0x4fc9db0c66c2f082u64, 0x6f307ed54053aa64u64, 0x59cb62092dbe9a02u64, 0x1a14bbe68151bfcfu64, 0xe27c127e538d9cc6u64, 0x7c30e8ee2442ab43u64, 0x7e731ce8e8198b13u64, 0x3bde0ed8b4540456u64, 0x7609e05ceb2ed44bu64, 0x312c56bbf0ba7317u64, 0xb40df90eea83c1bbu64, 0x11f2ae8b1882a5d9u64, 0x691c23f58a2c4d7au64, 0xcf805bae41533db3u64, 0x77d69d181088dcf7u64, 0xa2f59c0bb2f93f53u64, 0xb9675a5d6fddfdfcu64, 0xb557bb380b06d963u64, 0xd11366d0eb57e983u64, 0x9ef28573946c3167u64, 0xe33893298dc12e84u64, 0x3d0ed923a6578718u64, 0x456cc19ef4fdd711u64, 0x3a26743d513773a9u64, 0x730b197ac129c8a5u64, ],
	[ 0x707e22394de38de4u64, 0x96ae57dd4607abdcu64, 0x75c5793da84e66abu64, 0xc76152a6188e7db6u64, 0x8926d1230beaf2c5u64, 0xf0caa0bc0f6d9580u64, 0x02592dc41db577f4u64, 0x4691c50eca63dd19u64, 0xe84d607623daebe1u64, 0x9223a7e5e1e0cc93u64, 0x6f100f75e9438b56u64, 0xe734e42b4def792fu64, 0x3f604e975e7b5f9fu64, 0x61452b582de332dau64, 0xd9895e73e3e92f90u64, 0xa79b617feb1b99f9u64, 0xeb11870fdbf027a0u64, 0x4acb3b9e32234832u64, 0x6905a681a3d897fdu64, 0x51cd68624ba69012u64, 0xc0dcb40f5a6130deu64, 0x886b52c156b2aca1u64, 0xa1141c1b7b9168afu64, 0x9ecd68ec2ac7a3a5u64, 0xb3659749005ba880u64, 0x6a5d7a6b00599012u64, 0xd090fb8f7d8abd42u64, 0xccd16d5917155f21u64, 0x004d1e9b9e9dec89u64, 0x47fd94ba99b3bcdbu64, 0xb47d92ed051c14a1u64, 0x88146420609a6117u64, 0xec140f7d7e45e45bu64, 0xbf9f4a3904c2427eu64, 0x3e559b1313fadbc5u64, 0xcdb579eb292a8aa0u64, 0x1177436d67debd74u64, 0x81d4b3c2397dfedeu64, 0x1dbd0837320c704cu64, 0x2f1a5e763174164eu64, 0xe716f7fc76d0ffc5u64, 0x9bf0e689cff33d16u64, 0x64aa0945ee2b74ecu64, 0x58f2e7407bf00d51u64, 0x05d6715d1a29b25au64, 0x9039c881d8100aeeu64, 0x4e63060f1bb929a8u64, 0xfff7d6cd2655dc97u64, 0x0fb832d69eff6e9du64, 0x8890093d01d56315u64, 0x24ea50806624e544u64, 0x1e7d8bab003be4b6u64, 0x68e09a125459b3d2u64, 0xc5d88e0bdbb99f94u64, 0x5448f9a18e400f3du64, 0x49929b5d1272bf7du64, 0x4924f9ce0889b5e5u64, 0xdcaf52fbb5e9b477u64, 0x15363347072eee3du64, 0x71c8259d64054d9eu64, 0x74062115f0ae4841u64, 0xec9b0055c3257ebcu64, 0xdd82db9644404856u64, 0xa647505d5577296eu64, ],
	[ 0x43fd1f03d9533812u64, 0xc03dedeb21028bf2u64, 0xeacde1c73cae0c43u64, 0x81d32144dcbd7010u64, 0xe27dee4206482bb1u64, 0x51a42c9d7d77f412u64, 0x375299a8a29882aau64, 0x33b4760d28423c95u64, 0x47981eec6b00421fu64, 0xa413336b2aea0d28u64, 0x7dd7fb19182fd946u64, 0xa2e627ace70b7e82u64, 0xfaeccc6abccef6b2u64, 0x4723ba2f363adb59u64, 0xb3a86ecfc0f5253eu64, 0xfd8058e6132c1e59u64, 0x427fe0776d7313cbu64, 0x8d5c99bf793b11bau64, 0xe19902c18bef7307u64, 0x89ab75e9bc482697u64, 0xea656bdb68fcdd15u64, 0xd8e110646213cb2au64, 0xb5d3a3ae216da25du64, 0xa4197b92ef1af17fu64, 0x33952126c5ad4c3eu64, 0x788881132ac951d9u64, 0xa837c6b7d49b555bu64, 0x4774c1c04f72fd11u64, 0x5d3008caf7777167u64, 0x058ddd20bf283b60u64, 0xc2ba34a8d5be4fe4u64, 0xd387910d24c4d494u64, 0x16293790ae7a6327u64, 0xae5aeb7bfcce0840u64, 0x362e919a7eb66c81u64, 0x0e651b71a2f60693u64, 0xb7d4484a8381781eu64, 0xccdb8b98e01218c2u64, 0x2cbf320c20afbba9u64, 0xf657f9038f3673efu64, 0xcea9070a7706fdf0u64, 0x896a12987a9f7431u64, 0x40ddd6d037b7e30bu64, 0xdfda70dc9e9d57bfu64, 0xa65e87e2d4ce75c7u64, 0xeb395aa898cb2deau64, 0x9457fb8d04508a83u64, 0x769d5957493c24eau64, 0x50bcb99b951f364au64, 0xafb05d6f14a27ee1u64, 0x65bf430c8af59278u64, 0x493d6b52514c4de5u64, 0x563c36efe04b3434u64, 0x93ccb15af9620852u64, 0x6da59d2f5be9b263u64, 0x1ad9744ac4b06a13u64, 0x0b4e58075806340eu64, 0x3c08d2fd9d1f2228u64, 0x319d989d627595ddu64, 0xe3de7409c0ba3b5eu64, 0xc6d5dd719733f843u64, 0x970f6d5b60585396u64, 0xf17b035a46dcf3c2u64, 0x148cedea7264dadau64, ],
	[ 0x57e7c381559fd67bu64, 0x385c7dd41db249c8u64, 0x01285eb6e08b746du64, 0xe249421fbbdad773u64, 0x5c5146d464f5e3c1u64, 0x2a36ad78ac91c9e0u64, 0x6903e18fa7951d73u64, 0xfe68a8479c0f9003u64, 0x20c1b5692070d0c3u64, 0x1ae08c99ef0585f8u64, 0xe50ca595a89e018eu64, 0xb41c7cb5dfdfdc07u64, 0x8fdfe1796734e0efu64, 0x8a84af8c924a0366u64, 0xca0710eed5578468u64, 0x6d4f9443c9889645u64, 0x0fd5593ae137535au64, 0xbd609c5d0d03fabcu64, 0xf5165ee7963295e2u64, 0x96c839d53f794c58u64, 0xf399f30bfb5edf64u64, 0x49e2e744ed4593dau64, 0xc8dbfa670cdcd3b1u64, 0x7a80d559cfdb63f3u64, 0xba4288ecf9889f4fu64, 0x649fe3eb6c104ee1u64, 0xe5d09f58ef7a13bfu64, 0x5213b7dcb26343eeu64, 0xc94dcaa7e927dbb1u64, 0x67f268bd9863176cu64, 0x397059b4b7c521dfu64, 0x7bf3cf56c73004fcu64, 0xb5cc92e2ad9e7e0au64, 0xb7bd04b1392dd07cu64, 0x959140a2b524dc68u64, 0xa9de6e9d48303069u64, 0x9d5ef0a99bc912d8u64, 0xa44ada494d1fde43u64, 0xcaadd135f40bbc62u64, 0xca6110a15f4b3fb5u64, 0x00a3eb55fba806e0u64, 0xfcd95981c18b4049u64, 0xdb03aacff4adbbbfu64, 0xfd163a621a3d1ef2u64, 0x0f1776d5a4de6ed2u64, 0x6dd1e28e5451bbcbu64, 0x10c03ef88f24a314u64, 0xad0cfc0233c0accbu64, 0xce16a7711288bcf8u64, 0x4f07188461cdd99au64, 0x434736c66af57c16u64, 0x9a804facd60e6d65u64, 0xbf964e49b09e3f68u64, 0xb416f4622bbb8dabu64, 0xca5287336275ad31u64, 0xbdf7c047b9e6c5a9u64, 0x092047ae7106cf7eu64, 0xc8764fdb74ec65ebu64, 0x7edc5499d6b8b4b6u64, 0x52c05af063dc0837u64, 0xa373e3ae596b64a2u64, 0x188b9384842d0fb2u64, 0x9b2dd45876e5ad1cu64, 0xc3d458cd7f92ffa9u64, ],
	[ 0xe7cc33a7c89255f6u64, 0x7900d2d41bcfc6d1u64, 0xde04d2639abab70cu64, 0xc415d0ed4ed5a748u64, 0xd6cf960bfd690214u64, 0x29d345a504bba8f0u64, 0x0be11d56b59644e8u64, 0x5e160cc0d61de5f2u64, 0x18cca9551b1497f5u64, 0xf51635805e14cc36u64, 0x1ff0f6a15bf44e94u64, 0x18a3fc5e7657fb31u64, 0x1eb96f11698de580u64, 0x99f46f11fff86b95u64, 0x6effc7caeef236ccu64, 0x0f6ad1a646b1db5bu64, 0x588512e5144a21c3u64, 0xda8297d36c564872u64, 0x957d00d764ff6188u64, 0x8ed65ecd584112b1u64, 0x5052f0297ce29886u64, 0x80479296738cdf28u64, 0xaecd2b05da9c5f64u64, 0xdeed3046fef2bcccu64, 0x2c971ca300c77e75u64, 0x9fc87c93783737e3u64, 0x32ec656b6f6ae542u64, 0x7f1a3371446c5b13u64, 0xd1f6fec21a97b31au64, 0xecbfeba521d3ef65u64, 0xebc15b46807fa26fu64, 0x567abb729d1fc415u64, 0xb33be147d5884e5eu64, 0x45e9a7704054edb9u64, 0x7abfdfc16e3afcaau64, 0x7ce8db8863c3335eu64, 0xf29f7faab9be2df6u64, 0xd006d032dc203ea3u64, 0xaf3e402831c9d8b4u64, 0x60f714f0b1822eaau64, 0xd3e97a146f8edef3u64, 0x4248fde82891f90au64, 0x67d3dede45187227u64, 0xa86a27c07c5b50c5u64, 0x9d6b9134a964cd98u64, 0x9b1786bb008f0d9du64, 0x8731779c6312f476u64, 0x578e7811feab739bu64, 0xd923bc981e80305au64, 0x869d5e6c97fa7c2eu64, 0x3163d40ab81a9c75u64, 0x597f22455de36e1eu64, 0x04ae791ac195836fu64, 0xe34c783dc6dbb57du64, 0xf7f062a84116987du64, 0x737faddc75d461d1u64, 0x4ae4640217d2bf74u64, 0x398c63b12c53b37du64, 0x8fb3eb6b8f23c5a4u64, 0x39470a2c3d49bf7cu64, 0x0a87f3763384918fu64, 0x7974eec3256e222cu64, 0xdcc09a28dae22671u64, 0xe01fdc0292fdacd3u64, ],
	[ 0x4fc58936e846682bu64, 0xc04af9e3b10ded38u64, 0xb5740f0fadfe740eu64, 0xf1ccab83c3619596u64, 0xb5efca5984f3dcfbu64, 0x003aab1765b287a5u64, 0x749bd243bf38dc7eu64, 0x64657eb322604465u64, 0xc3d2af2af7140be2u64, 0x376572b171d1763cu64, 0xbf4ae97de18ba43eu64, 0x6104f6f6db1677ebu64, 0xf82646bd3b365cb9u64, 0xb58a0cf145986794u64, 0x0955cec3c307faf0u64, 0x4c966bd051f69de5u64, 0x58be094c2fc21047u64, 0xddecd19ec361384cu64, 0x2dcb138ed2d84116u64, 0x2383518175f059b3u64, 0xcd08f6577a4c702eu64, 0xa12bcbaa84309116u64, 0x3877813cd3ed70e9u64, 0x815acbdfba74a174u64, 0xe200e33de21ddee2u64, 0xc29eaca82394fb00u64, 0x5bdfb5f7e033fe9du64, 0x2651e165fee1cde2u64, 0x40a6443de16f4498u64, 0x383f15edfb37d3ebu64, 0xb613c66189165852u64, 0x6441eb89f4b9d784u64, 0x57eb069435998af1u64, 0x737a4f6d4119d84fu64, 0x07e91266572eef01u64, 0x62355879324c1ebbu64, 0x40ae8e7a36f32d32u64, 0x141eafa611d4cb53u64, 0xc339ce46a7026ce4u64, 0xfbad2f34e6a30ff5u64, 0x51b08012b16f20e5u64, 0xc62b647b86e1ee10u64, 0x0edc5dde5d24d507u64, 0x68e30408cc49acdfu64, 0x0a2a21142ef5805bu64, 0x0e3905b30873a438u64, 0xda613402c03b97d3u64, 0x567abc976bbc7a8bu64, 0x5ba9ff32ffb93803u64, 0x298e201d07173cd0u64, 0xd15b6390b1a3cc8fu64, 0x23505d8cdd6280bfu64, 0x2ea7c2e7bff21d84u64, 0xab751337026238edu64, 0x62a792d27add5a77u64, 0xeef784a54b7353a6u64, 0x49deff441bfc0e16u64, 0x59853ad079c7a1e0u64, 0x08ec5ef3987760b2u64, 0x21eb41816258c42du64, 0x4a2b88d749362da8u64, 0xb946c1985fbfc20du64, 0xa080ae9a3603c8b3u64, 0x7b29e3463eb9e087u64, ],
	[ 0x94b9e9f5ceb52b5fu64, 0x8354afe16c72a7d8u64, 0x990e437434801dddu64, 0x0f4dee3bbe353193u64, 0xacace5641d0104e3u64, 0x79162f154285cce6u64, 0x9a900a8294e4ef3au64, 0xcb347ee06539a65cu64, 0xb3938dc56f99ee9au64, 0x683b285b94780a9eu64, 0xbebeacbab1dfd045u64, 0x3523413b3c7bcb58u64, 0xdea3e81105400074u64, 0x189f694134020a2bu64, 0x4f3fce034ebee954u64, 0x3546693984e1998eu64, 0xf8184a09436e588du64, 0xd4d59d1aac43d77bu64, 0x5f48e031d44107d6u64, 0xef62813f9859fd69u64, 0xf8361b4201c3e5cfu64, 0xe8097151d3e28e31u64, 0x396ffae740871dd4u64, 0x746cab6d74f26694u64, 0xf4939542dd5f96eau64, 0x2cf885446a024ca2u64, 0x923819ac7cf15086u64, 0xed38932492cda2abu64, 0xa5b8dec02f01814du64, 0x5391062166cf4e43u64, 0x0c2e79ac5901a400u64, 0x26580dc2e0c42b60u64, 0xc270db96e4d6ac34u64, 0x2e3fb389f3706d39u64, 0xdf75027866b53381u64, 0xea3f29015732506au64, 0xc6f6ee8136621fd9u64, 0x0e6e97c108dcb11fu64, 0xb963cbb500d4990bu64, 0xa6e4104bbe4c5685u64, 0x1caee1e98d4fe983u64, 0xcc4c09466da163e2u64, 0x7f671df532bd3b36u64, 0x21dc2a5c4efa1b03u64, 0xbc811dd372f48696u64, 0xbd9766a4f5d76cffu64, 0x802d5c394d7c49c2u64, 0xdf4b91408044ea74u64, 0xd99e49005fe3d172u64, 0x33b1608009adb58fu64, 0x21fe81baf47ad7a4u64, 0xec76062d7196635cu64, 0x743f04e03c7066f3u64, 0xf1db53635c77d312u64, 0x26f69f7a22e85ffdu64, 0x94b970038e3d14d4u64, 0x7869668a3f33eccau64, 0x27f1b8374e5788b5u64, 0x658cf407d9cce32au64, 0x41898bf90fe4f165u64, 0xdcc55b4a9fa4352fu64, 0x56a04e3d1231011eu64, 0xf77cdb2a0fccabd0u64, 0xe89d831351478480u64, ],
	[ 0xc2121b781efd554eu64, 0x6a7201307fa64e9cu64, 0x03051fcfbe295500u64, 0x6e1e97ddb8207787u64, 0x2e3c01798dcd3d4au64, 0x73a85a6477b123ddu64, 0xd9082ba5dac938c7u64, 0x56e5e400fbbd0f84u64, 0xbff85c30a7209111u64, 0x94b05e3b3ad53df8u64, 0xea7e003ce7f34becu64, 0x4272a3a9e37484f0u64, 0x3d32761c9c59f76du64, 0x25bd227242acf101u64, 0x8beb46b71c81095fu64, 0x08e03590f0319540u64, 0xc72635a09835575du64, 0x590df572fdb13637u64, 0x983534636dff4512u64, 0x54059e1ace7bcb20u64, 0x58fee535374ce4e5u64, 0x2dc8da3dda1a6035u64, 0xc7e88bc613d22862u64, 0x6031cfb1c7b06dcdu64, 0xba4311227b31a10bu64, 0x72093c94360ba42du64, 0xe3f6f1a68db97ca8u64, 0xa880a0a247b7bda8u64, 0x770d50cb565f8da5u64, 0x3542fb57c5fed06au64, 0x2b9eb96abe39b440u64, 0x2472d8481e69d860u64, 0x53503a37f1029dcau64, 0x22bb550c4cc4b578u64, 0x3137169c8bb9ee91u64, 0x9d3280ed64858fd3u64, 0x6c9a008ecc7eb541u64, 0xf2d7e90879a33dd2u64, 0x67edc649cbc8af21u64, 0x72169c201e2f4ab6u64, 0xc49a0e8793daca11u64, 0xffd0baeeeb1e0794u64, 0xa5e8a38193061ad7u64, 0x08b2e0e1dc024d3du64, 0x348a1f1491ba2f88u64, 0x32656930754cf708u64, 0x79efd96575d8e314u64, 0x22adf5dce3748f5au64, 0x9ac96e7918567b88u64, 0xa0b5d880a0475830u64, 0x727d50948f3f9cdau64, 0x2cdee1f3349d2014u64, 0x8aacc9dff7670b6du64, 0xadf35399ccbcc92du64, 0xe2902979d64e1515u64, 0xa271b87aa2380371u64, 0xecb74bae864110bbu64, 0x705707a5307b6a4du64, 0xbf632b7fbc09ba57u64, 0x8a5c7bec8e0dc153u64, 0x1f168e83a65a551eu64, 0xeea93e54690a65eau64, 0x9a5868edafb11c76u64, 0xa607ae2a36823573u64, ],
	[ 0x3d2b82afffb4c997u64, 0x1c6a0994c082d9d6u64, 0x91dc5dd3c2d78325u64, 0x52d515c86d7bb037u64, 0x9be9a5f065cdab5au64, 0xd6c485f046e4225bu64, 0xf887d718c09b0e5au64, 0x241fd5d23b666b06u64, 0xd9aebcea3541b64au64, 0xe864502defdada2du64, 0x2ac56b78d58b6403u64, 0x2422358ba863f952u64, 0x0581041f986ae9afu64, 0xb6020c72c4dfd51cu64, 0x153bf7e9e80f09dcu64, 0x6a794821becd2074u64, 0x58d3897db2bf6fb0u64, 0x12711a11a78eaad5u64, 0x29a5ae1b2d901322u64, 0x029aef9a04962347u64, 0xcd344b67dd29eb9fu64, 0x3a63b59dc40ac963u64, 0x4ed950212da3fde6u64, 0xf6a6d1a44d922e41u64, 0x1025b8ca4447f772u64, 0xfce08767b7bed312u64, 0x956b248d7d925876u64, 0x1a0a89c04ecf62fcu64, 0x54c02abfe7d6c4d0u64, 0x2910997e67aabeeau64, 0x9bd67ae83d9b3c2au64, 0xd9cd33f2213c78cdu64, 0x1357c01914decba0u64, 0xabf22c3aeaa854eau64, 0x6310e373393f1e9bu64, 0x48052afbb62dc57au64, 0x8c825c5b635cc654u64, 0xa1aca0372f17e94bu64, 0xe27a2c702ec7d6c5u64, 0x4a9a2db6635dbbb4u64, 0x299224314dd5450au64, 0x4744be0d23aa521fu64, 0xef4601e916488c95u64, 0xa5435881a7ca073eu64, 0x2f95df501c615167u64, 0x3e4a454b91192e99u64, 0xa5485f4adb41599bu64, 0xd3132547c905e781u64, 0x82b3203a63a35539u64, 0xdfda5d4e5faf65bbu64, 0xf0711e373792da08u64, 0x4c1681fc50e5327cu64, 0x1b2d139649744ed8u64, 0x6c17d5c2c62e5f0eu64, 0x7021f3a7d1e073e5u64, 0x84804655f372fb6bu64, 0xdfc011c33a16b053u64, 0x44bb2e58fad9f3b7u64, 0xa4e87f66ae4dd5ceu64, 0xa57f025fc801514fu64, 0x8e313d1cbd7eada5u64, 0xa31529b0b6af3fa7u64, 0xa51a263e08c2a417u64, 0x0566ed041a02f843u64, ],
];

const CASTLE_HSH: [u64; 4] = [
	0xf1b455e8b32ba0bau64, 0x7a7b0a46ea54664fu64, 0x1b3cedc499063c95u64, 0x125b2ab39fbfbdfeu64, 
];

pub(crate) const EN_PASSANT_HSH: [u64; 8] = [
	0xc748843721c0ccc1u64, 0x6e64045c3a47b82fu64, 0x5541cde174a08dd2u64, 0x76787d176b0c1412u64, 0xb6a9d18d16369af1u64, 0x02d4431d10dfc13bu64, 0x13aa0ef5bf11f423u64, 0x55b8eb025e2c33f5u64, 
];

pub(crate) const SIDE_TO_MOVE_HSH: u64 = 0x189fb87cbba377deu64;

impl Board {
    pub(crate) fn calculate_castle_hsh(&self) -> u64 {
        let mut hsh = 0u64;

        if self.board_state.castle_rights_white_left() { hsh ^= CASTLE_HSH[0]; }
        if self.board_state.castle_rights_white_right() { hsh ^= CASTLE_HSH[1]; }
        if self.board_state.castle_rights_black_left() { hsh ^= CASTLE_HSH[2]; }
        if self.board_state.castle_rights_black_right() { hsh ^= CASTLE_HSH[3]; }

        hsh
    }

    pub(crate) fn compute_full_hsh(&self) -> u64 {
        let mut hsh = 0u64;

        for idx in 0..64 {
            let piece_color = self.pieces[idx];
            if piece_color == PieceColor::None {
                continue;
            }

            hsh ^= PIECE_COLOR_HSH[piece_color as usize][idx];
        }

        if self.board_state.castle_rights_white_left() { hsh ^= CASTLE_HSH[0]; }
        if self.board_state.castle_rights_white_right() { hsh ^= CASTLE_HSH[1]; }
        if self.board_state.castle_rights_black_left() { hsh ^= CASTLE_HSH[2]; }
        if self.board_state.castle_rights_black_right() { hsh ^= CASTLE_HSH[3]; }

        if let Some(ep_idx) = self.board_state.en_passant {
            let col = (ep_idx % 8) as usize;
            hsh ^= EN_PASSANT_HSH[col];
        }

        if self.side_to_move == Color::Black {
            hsh ^= SIDE_TO_MOVE_HSH;
        }

        hsh
    }

    pub(crate) fn handle_move(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit = (1 << piece_move.to) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit == 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] == PieceColor::None);

            let piece_type_from = self.pieces[piece_move.from as usize].extract_piece() as usize;
            assert!(self.bitboard[piece_type_from][us as usize] & from_bit > 0);
        }

        let piece = self.pieces[piece_move.from as usize].extract_piece();
        self.toggle_piece(us, piece, piece_move.from);
        self.toggle_piece(us, piece, piece_move.to);

        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.from as usize];
    }

    pub(crate) fn handle_capture(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit  = (1 << piece_move.to) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit > 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] != PieceColor::None);

            let piece_type_from = self.pieces[piece_move.from as usize].extract_piece() as usize;
            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece() as usize;
            assert!(self.bitboard[piece_type_from][us as usize] & from_bit > 0);
            assert!(self.bitboard[piece_type_to][them as usize] & to_bit > 0);
        }

        let our_piece = self.pieces[piece_move.from as usize].extract_piece();
        let their_piece = self.pieces[piece_move.to as usize].extract_piece();

        self.toggle_piece(us, our_piece, piece_move.from);
        self.toggle_piece(them, their_piece, piece_move.to);
        self.toggle_piece(us, our_piece, piece_move.to);

        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.from as usize];
        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.to as usize];

        self.board_state.captured_piece_type = Some(their_piece);
    }

    pub(crate) fn handle_en_passant_capture(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();
        let victim_idx = if us == Color::White { piece_move.to - 8 } else { piece_move.to + 8};

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit  = (1 << piece_move.to) as u64;
            let victim_bit = (1 << victim_idx) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit == 0);
            assert!(self.occupied[us as usize] & victim_bit == 0);
            assert!(self.occupied[them as usize] & victim_bit > 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] == PieceColor::None);
            assert!(self.pieces[victim_idx as usize] != PieceColor::None);

            let piece_type_from = self.pieces[piece_move.from as usize].extract_piece();
            let piece_type_victim = self.pieces[victim_idx as usize].extract_piece();
            assert!(piece_type_from == Piece::Pawn);
            assert!(piece_type_victim == Piece::Pawn);

            assert!(self.bitboard[Piece::Pawn as usize][us as usize] & from_bit > 0);
            assert!(self.bitboard[Piece::Pawn as usize][them as usize] & victim_bit > 0);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.from);
        self.toggle_piece(them, Piece::Pawn, victim_idx);
        self.toggle_piece(us, Piece::Pawn, piece_move.to);
    }

    // updates piece and the .to
    pub(crate) fn handle_promotion(&mut self, piece_move: &PieceMove) {
        let to_bit = (1 << piece_move.to) as u64;
        let us = self.side_to_move;

        if cfg!(debug_assertions) {
            assert!(self.occupied[us as usize] & to_bit != 0);
            assert!(self.pieces[piece_move.to as usize] != PieceColor::None);

            let piece_type_to = self.pieces[piece_move.to as usize].extract_piece();
            assert!(piece_type_to == Piece::Pawn);
            assert!(self.bitboard[piece_type_to as usize][us as usize] & to_bit > 0);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.to);
        match piece_move.flag {
            MoveFlag::PromoteToKnight | MoveFlag::PromoteToKnightAndCapture => self.toggle_piece(us, Piece::Knight, piece_move.to), 
            MoveFlag::PromoteToBishop | MoveFlag::PromoteToBishopAndCapture => self.toggle_piece(us, Piece::Bishop, piece_move.to), 
            MoveFlag::PromoteToRook | MoveFlag::PromoteToRookAndCapture => self.toggle_piece(us, Piece::Rook, piece_move.to), 
            MoveFlag::PromoteToQueen | MoveFlag::PromoteToQueenAndCapture => self.toggle_piece(us, Piece::Queen, piece_move.to), 
            _ => unreachable!(),
        }
    }

    pub(crate) fn handle_castle(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let (rook_from, rook_to) = match piece_move.to {
            6 => (7, 5), // white short
            2 => (0, 3), // white long
            62 => (63, 61), // black short
            58 => (56, 59), // black long
            _ => unreachable!("Error in handle_castle: innapropriate castle setup"),
        };

        if cfg!(debug_assertions) {
            let from_bit = 1u64 << piece_move.from;
            let to_bit = 1u64 << piece_move.to;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[us as usize] & to_bit == 0);

            assert!(self.occupied[us as usize] & (1u64 << rook_from) > 0);
            assert!(self.occupied[us as usize] & (1u64 << rook_to) == 0);

            assert!(self.pieces[piece_move.from as usize].extract_piece() == Piece::King);
        }

        self.toggle_piece(us, Piece::King, piece_move.from);
        self.toggle_piece(us, Piece::King, piece_move.to);
        self.toggle_piece(us, Piece::Rook, rook_from);
        self.toggle_piece(us, Piece::Rook, rook_to);

        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.from as usize];
        self.board_state.castle_rights &= CASTLING_RIGHTS_UPDATE[piece_move.to as usize];
    }

    pub(crate) fn handle_double_pawn_push(&mut self, piece_move: &PieceMove) {
        let us = self.side_to_move;
        let them = self.side_to_move.get_opposite();

        if cfg!(debug_assertions) {
            let from_bit = (1 << piece_move.from) as u64;
            let to_bit = (1 << piece_move.to) as u64;

            assert!(self.occupied[us as usize] & from_bit > 0);
            assert!(self.occupied[them as usize] & from_bit == 0);
            assert!(self.occupied[us as usize] & to_bit == 0);
            assert!(self.occupied[them as usize] & to_bit == 0);

            assert!(self.pieces[piece_move.from as usize] != PieceColor::None);
            assert!(self.pieces[piece_move.to as usize] == PieceColor::None);

            assert!(self.bitboard[Piece::Pawn as usize][us as usize] & from_bit > 0);
            assert!(self.pieces[piece_move.from as usize].extract_piece() == Piece::Pawn);
        }

        self.toggle_piece(us, Piece::Pawn, piece_move.from);
        self.toggle_piece(us, Piece::Pawn, piece_move.to);

        self.board_state.en_passant = Some(
            if us == Color::White { piece_move.from + 8 } else { piece_move.from - 8 }
        );
    }

    pub(crate) fn toggle_piece(&mut self, color: Color, piece: Piece, idx: u8) {
        let bit = 1u64 << idx;
        self.occupied[color as usize] ^= bit;
        self.bitboard[piece as usize][color as usize] ^= bit;

        if self.occupied[color as usize] & bit > 0 {
            self.pieces[idx as usize] = PieceColor::new(piece, color);
        } else {
            self.pieces[idx as usize] = PieceColor::None;
        }
        
        // hsh update
        let piece_color = PieceColor::new(piece, color);
        self.hsh ^= PIECE_COLOR_HSH[piece_color as usize][idx as usize];
    }
}