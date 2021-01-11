[
    // https://tools.ietf.org/html/rfc4231#section-4.2
    TestVector {
        data: "4869205468657265",
        key: "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
        mac: "afd03944d84895626b0825f4ab46907f15f9dadbe4101ec682aa034c7cebc59cfaea9ea9076ede7f4af152e8b2fa9cb6",
    },
    TestVector {
        data: "7768617420646f2079612077616e7420666f72206e6f7468696e673f",
        key: "4a656665",
        mac: "af45d2e376484031617f78d2b58a6b1b9c7ef464f5a01b47e42ec3736322445e8e2240ca5e69e2c78b3239ecfab21649",
    },
    TestVector {
        data: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
        key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        mac: "88062608d3e6ad8a0aa2ace014c8a86f0aa635d947ac9febe83ef4e55966144b2a5ab39dc13814b94e3ab6e101a34f27",
    },
    TestVector {
        data: "cdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcd",
        key: "0102030405060708090a0b0c0d0e0f10111213141516171819",
        mac: "3e8a69b7783c25851933ab6290af6ca77a9981480850009cc5577c6e1f573b4e6801dd23c4a7d679ccf8a386c674cffb",
    },
    TestVector {
        data: "54657374205573696e67204c6172676572205468616e20426c6f636b2d53697a65204b6579202d2048617368204b6579204669727374",
        key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        mac: "4ece084485813e9088d2c63a041bc5b44f9ef1012a2b588f3cd11f05033ac4c60c2ef6ab4030fe8296248df163f44952",
    },
    TestVector {
        data: "5468697320697320612074657374207573696e672061206c6172676572207468616e20626c6f636b2d73697a65206b657920616e642061206c6172676572207468616e20626c6f636b2d73697a6520646174612e20546865206b6579206e6565647320746f20626520686173686564206265666f7265206265696e6720757365642062792074686520484d414320616c676f726974686d2e",
        key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        mac: "6617178e941f020d351e2f254e8fd32c602420feb0b8fb9adccebb82461e99c5a678cc31e799176d3860e6110c46523e",
    },

    // generated using: utils/test_vectors/py/main.py
    TestVector {
        data: "d74021fce1a1a5175e9463c6453f69fb8017f100e28e76cde969b17d58b98ad6cb3df9d60254b048a594c81a7242e7ad8f9030b9827acc786d3e12b0a28372f4d4ecfd7791751311dbc4e4499318114cb643d23cb5fcf80e8940a355c98512995166c20a92bbb4c50dc366e42af8ad0634a1a95029ccf392b91796bf1772e65909ff117e01c19229651ee8bb7979bbf9b9a4d87e740687bfd224e0ec5587d39be8f9d4279f8849a18a863a5cb71f1daf6db9771ddff6811bcac2a6935ca35b269c06bfa2405490bb831733336c1139940a8375f490108c7bda9777c3f41febea14c227e1301447a2ef430171d6bbb0e4ef7ba8aeaa74c2d6b402d575195f27340d0e3bcd1d3c6b787c3b93de19ecf17e9d093408dee20def2eb63a758f7a4fc0f6cd13c22a884772a16593a8595d23783c38eaf3dd83fb154770697165d16f841620f93572156f64dedd35c56af34d209c091736dd134cbbdf444c92152e4c3bed864812343ed97c3fc30d6bb9aaa6a4d7113abde534843ad9d1d4cf590f7ae57358698a026f9e9a1c00608d23bebcccc3c82ce916fbcc061500dcd9f8a436cf16f76b7ef2f48bbe8b5dec11fbeee22b63c1e285370a48fc7fcdaf73dabee32df7bca4876cb99ccc715aa98f61d73d9aeede3c5b1b901fc094be7c859dd01364ea68f03bcb04af2e939d4bf5468f5fe1e97570014ad0d8913a35a155c9ec3bc68481e676411da547c0d530e14ce60aee88e72ab56a9e6598502fbbef85691eaea7f1034b104e675bc84b19cc53b5c2b701a00d7542bd1027452ced8396610edc561e0ec1c7e2d382fb5e4f6fa3bb0eabcb0cc7beb27ec642d6ce54a8a0cd85",
        key: "775d9904",
        mac: "c6eda655b90dbdea90b1bf3f9040154fcd3cbdc286bdf10163d7c46922c33fbba676be43fa9c2cd08bc1ce507d591fb5",
    },
    TestVector {
        data: "42469412c7ab75c8de1164ddfade1cb667ae76d7bd5ff30d189add7d1473677366bba0583793b365f1f6a40ccea5db703ab2d1a7cc3c0641504bd34abfd21c8712e4222d3da102abed15a4ea5ea1874baa5969403a8b190fe557427cc5bf4b8002e32c5c7906b9d55bccf8dc560c34c26ac527994bff2246fae017f687e7918296de53f3e9a658afd610671fc469e19ca8c63df55f42d35c1e55622afb667b8781647ae973334530a7cf3d2995ba79a70375a29bf8de4e14fb89bddb8fc707c4612033088e6e3311504d4880c0e45c508e9cd69147cbbb24665fb77f815ea4118d6dc56107cea3e2fca4c469547fe5ed485f5bd9d9c53c3f901077cc4727fa4f754a2468e238cb54bf3ec29adb2fbd1a5ac04478fb9c658d90ec0d6d3970e7a965e1e5fe6c68b89b52c289d872215d6e5ef6ddf15f1f8a75f01aea4bc93660b3c7992bb7cac4f69f237938c4b46000f267a99912fc7b86312a889d535984a98221ee598bc7cef8105f520dd9fafb69a884ac54b6208b7770482b36866f9fe1c543a83b5b51bf74fdacec8bc027e36e7457a581b8c76e63bacea505ed96bb30fe439b60b2122db8d01ce39776de814ffbc2f6e36e0b2f9e50b3e364cf1c24e18a5d3a1a6143c6eb68d26176dad45d4b25cb2d4de839f2b3eeda2822aff5ed8e715707917fa5fc4fa754d16a0dc4612075333be752ea9c5535044bb87f0dee3fca428227ff2e191cf9cdbe7b82253cf89dec5cb977e15bf63515bb7e92a565f843e3c2eceefdc5271644aba7be9e15af054e9999e87278c277c9fd382ec0f75c56b5b8800aec6e9efc7b8c43d128483b5d5b5207ab3ffcd2f313202dbcf3c59387373fc6826aa3fdc8deb55e04d0659cd4e33ed81225f440d1edc0d5fa0ca9089f548a1cc1632491c48d87b489d4353ba2a050d9b51d46ca5f947822621b1435568237881b2dedeb5f",
        key: "e929a5d6ebaf779611c0614cd6bee29b123ce13560e581751d921c7d43c9038fde877bff54d13ff3f624169f532b1a720ca1272a37103891fcb40638dfb2e64fc58fad1a461a851cf41da8d493def737ebe85f284a6b4d7c5a5314c453c42e830ab4c292d9a4c4ae3b43b019a10b17f78a53d78ff1621b0302221726c6d70ddb6d83ccebe74aef775b63f73116cd4740b8328ccae8c20627017bac904ceeb64cf6accb189e63c1ee674b4e697b09879e825a5239c84d9641893e329e8ea4c93e9c35b0a4e0ebab63cc6c4eb4e114dbb6c4030c8d0b82c6eeaf0644bdae61512b384b0f0fb2a9ebcd1813abc7c3659a730d2db1a06dab2934592ebca98cc9acc6d7aabfa2049e1c42721c6617a7ef5123db078215939a7d537ff0e65186ea59a2b4911373b9661e558a15301c16b542aacb6391a9ec4b156d7f079cc6542938ee624e89ea8dced036a314e44368bd93e254a6df721dc4797ce040045b6a0642a8357639ce031a1e98a2a6e12855e7654dc48f2979387498ed5778de8369dd2c8716d991d13c4cb00dfc919c28cc09ecc2257a7ed02bc14e9092a1c7bdc7437cbedc1d4bbdae107880f8170652f7e42657355e418f237bca062f7dfcbcfcec035ea8cc5228de92b972b1b90acbbce9b3212d3c5465a99ffb2866c891ebc75264ab52517e453ab19364d99c7a7d0946d39cb79fada1a788a47ccfaf86f8cbc94950a11a20713ef911bd47c16ce1dc4b29393fad79b4c2d6d1559303ca2ccc116ae7e1d58ac3efc5828f1b976f05efc4faa0db1e44891f76d2f0b951067a384e5557525d39f673f59e5ffea93a8b032f63ef0accc8abf1018c27a97d40f8c71fad4bd34e385bf1caa53c8ea7afe00af4903b5f5c65b4ced7d62bc0f78c01f58248b15d7cf4a33048df78975795e3275e8ccbed0e1d853bf3812cdd169dabcc78947851585702eb8924d4cabb7b3ced27d3357c08733ffa4e22fdc72b5b211edf9e0add3c9ca732fe7a9cbb95dbd646fb09c5e99db56a67a52d4915ca00766e324089dcd2c9d988b5d40cfe0c0b27e132daa30165870c6b7408e829ea188b21d312bbd18319e292122117a09a0b8bc377352f77406ec7fee553d5c325c8ce2ae3a2e6603f6e201e4520fb8d3109fd4637e70b6223b45c1783eb76fb17cb46f5a6efe2c82108e63b871478ac289cc8a7da76ef06fb2191eeed76154a2013c1693cd2ad4ffd3c6f6e09b5",
        mac: "8e7c66c7fd9aa83f979559f46ab97b84a4451e01c96a5a76266bf1c8471894489c325431212d3710c79da61a3b53645c",
    },
    TestVector {
        data: "d2641e10df0d1941bfbbd0abc1f61ce1d5ac980cb5be4e685687",
        key: "03fed297f3bf88e3db03c9ed7efb93be3dcd79fedd1c694116096dd98e9d592e90fcfa7f38eb424717f20d1d1208c1ca79744daa9c0cd8909ce034af6066cb5ab9b6daaa4127353061459488311e3bee478c8c79e711d84613842b7d2140eb85f7309ce0a3511fbb42f1cb68537b4d618fdd6ff5ce0dd7f8552883f0bb549868136000b2549e943493f68b813b5a6357fa7b62bcd4627b7f21bb0a5ca970b4a90c28d02f6440a14f0453a9ac7f8eccdec55656e086c02bfc488ebbdafe107c7cfc43e982687b4a9a9b8b7148328559f0360a4a22d05e5d3fdece30914b424ddc0edc4543ffa0db34275e3cb944fd7a805abe5ed29c9eb217c4b95d2ef8bd37ba086060778868e4b177ebe0ee56e840c9c6c48de5881fed4bda455599e4e0c92d847a9de43752f3e200c13be93e7b4ec28dcbef1e4535c01b95b971dee45aa0338f64e765831324037dd21b9314fab26dd506ff1a43b278b85553e316f182c9504b63d3cbba3868cf20d930edfa79b7d0e8ddf2257166c2195837c942b0d24144733110fc11da194e9f2d7047056596dbe92d0e0afab9d262c304ede32fa5b2b7004b6044ad02473649def29c4aec8abc3e2fd986822ecedf1e70d96cdc2df18c6363d47860973e95175435d9bb2393fc8f5e1b2813ae52452aa2e9c08a48479425aea4c46419ec2fbaefe8a061288aca7a8952f76a799ad7cfb2182c42d9bf611b70e67fc8f88a460f75ba3b5b630e357caadf70eeb2d56c7ce212dcaf8d750b20facf05e3ec941658bd947e4b7dfded583ba1e50fbcdb5cc31e9298c9ef42fafd50740e51e0603ca2e4d8c9f1d0fcaeff934fd3eaa7e2b049aae2d36fe059a2999658cbbab4a582816ef284801a87531178a4922497bda990112e3070a5db77ef973edb60ac1b7dd88292bc595c822d1283da7ec0e1d5f02dede84bd87b99d9f035a8cb5a24a097f7e7d6258a237b382dbc6c553b5b9f54b7f82e4e85f68465149f8d999a99e8e431de8987fe07b032716d1a0dcba33e1fb2b7c72da0f63e55556ee6db0c1a9e90693f9115e7e49542711e441c11b2ed35465afc8802b8f8a4311c609ffe45a63c9f7341f3a6f7e3925294624e7c2112",
        mac: "0e06f2b681c8f8775cc3dccb4ef575eda3d6e39576b0dcb5bca33fef696d643cf6c5593f8e31ea09e1208cac0feaaa28",
    },
    TestVector {
        data: "ebad2b3f39f51545a87f9fde91559f0669fb1894047f9e78b14a694f479d4b67edc94d593fd270b39aaa72e231e2964507d6a6b3421304d238ff5b0a93b9826dabed66d6817d5ef04bb95c91b54e3619724fff025280ce53fee70030672d5eb0be5d58879b1725716d32ab5ba6ceb142ea88502c171fa4679570e96c929d29968c2637f3749757d4fff31048552a277e7e700698654597770a9f34d864bef78165f76f44fb6898cf09c4d6405ee3fe0da1969a7fd0a2529881ee72dab4ae2e677ec1a8319e70be9faeeefc6f5e5b1f60cf45c8197f6b4b452272f9062027e28f46381256abf4e0768021a9f5905b842d85540d2346f3f66ccd27faa9c5622a467845b7f21920a7cfeddb78a846138346cb4cc697",
        key: "04021597cf80d1796be68a95b760b28cfe789dc3c697e37eb3b721497fa50d9ff8cc88b3b78dbdec6e75a2235c685c719534d1650c733fe19413fc234ecd23121daa8eafd1e0e88c783128f2aa4a5d93f44b85a33ec96fac86190fa0393fb4867f4c213d4a6e5b3c021c30e8671fdd1e973c2dfbd35f455d296e901965cbdff567bc71a55238bfaa2783c7d14e262a45074f128b053c8a6f564d186a241de404b193f7b9301f44e6561dbba17c0cb0ec443f973a17e4ffc2d96a08ff6df9bfcc87da143a230075aa9151b8fba618a847774a251f2aba84809de664b81dec30492b6b51c3b4d99aa1244b4d8ed23be8aff1694d974fbb268e8b145cc4ec2651abcf0e218af944fe717892590a1dbe04aec329678a12864d7ace6e7ede996aff98366e2e2a0b1136b522595cc42714b6c40261490f5bc9eb46fe72dd28b77446865d45597154b18ac214263ece614b7cffac944abcf68894a6182181",
        mac: "58bf6cc62528891fde44fb20e39f6231d988e9f02fcfcb29bd0c867b500d25ad08590ee0cd2167dc028522fecb8c19bf",
    },
    TestVector {
        data: "6029f207326b02b14b71056efb52731eb67ff8b5ef565d2d62ed828cd6a36db4910d55e7cecb1f99130e0e99bde63b7124d1e2f18220aa60f2bf922cbc2c4f279e162a2e7ba40f6e528929dc79b12b4bdbbbd0d5d0f92cb18cf742a4e55c071f1830161919a057beaa84828f5bf8ab5db1a80ab2f8e06540b9314dbf2b109c4c0d9fe231d9b3b02b14c065d6bb396f1805150cb80e3dc9847cc59fe4dc09a732defa5159ee912ea68e5705da81edace26922057eda5127e23e90e706863ae852ef883bc6a4fb2568c498771fe8dc700c939177955c2511a5ebc23328e7ae269f8eda156073d4c873a88a97d932727a1d5560d90540d08dbaa04a85fe16ccfeed1af730786497af524c436b55148aecaae05c38cec6da8875f36460ba156f346d3f5f6d47eb9a31bfb69fb93a61bef5f9312a8a1ae759f45135d3799c8e5aba06d2b656bca18f7d9497cf8a8ac5a1f60cc5643017c2631e1fbef448caa55e2344df715feed0ea206d00b0be1fffe2aa008b6a60da7242d747f0092ac9a5880645b7e284682be86bd08cd3bcfff6698bfa6d2bececb55a26cc87efc65e69984bf841e4f6e57a8d01e21c1caddd49fb7939aec7d45a4267e9688dd5a9a564ce30d48d46ecdbd0147272da7891b17fad122fa80180e9c00adfebb22b0a6f4f83fe499f7222d32c2da04caacdce0520e31e5b488730c48dba68d70aa52e2d80b90ce19db230653941c7a546eee09db32ed90375dfe6edd0099caad97f539f0e8678103953747e67a6560e33a15d35b09b8ce7b5a9b5747e1a0321b2b6989cde67851480ef369e0420fbcf4611a717eee9ea76f1ecc5796e4f9497c6e7a1877d04418626fd9c83b1a548b608e7b2f15c194d0ac246500ec81b279c5ca34d9229dc9a36a5f47884ef5df4e27f7f95d7045591a9d2c037cf3e3b750fcf31f3cc1a3e2e4132894cb5085947774408098a1369fb2d70dc90a1d8466204457eba6f1e0b107b9a418b563b488d1ca342ec2692639383c54a0b9f90cb21bbc9c6b4339aac18d74f0e67336493b77524d6",
        key: "985f0e28fafc3c21c02d39c18aa39f990b381b16b11ca22cd385c070095eb6e9c422d3c8f15d9df6755a242d14cec6a279886ef3e6f15b74497d11cb53a637ae64d0ffba0eac092e24fe1a847419ab63cb00d0fc0a0c20d136561c2f6663c0258d337daa6d9270829b7596ff3f8457d69d277f063768ef697fb3de85bf626c7d27a01b3726c1bed75ab501c4945d487d7bc488d644a7b81bc69f045eeda1e9bb30653e8e072ca7bb02c97af70ae340fa8c9cc3ec21d9e0263f41ccea99d1e36991a6f5ed152d76ab8c2c11e13ac1fa784d52ada49e2bbbc20149bf9afce9e9f9cfb07022db615992ea14dd405a4f7445bc6b9a6c13a4d9537ba13a680581c31041b9af8a63a9c216440ca0b774d46794e90f8b55cdac16780d36ed42b344dea9ea26058841b21575e23c80aaf187e33f12386ee3b299c1e2286055bff2144078de41aab6bf807ebddeed811e8e11c545a5e32bc216f64dc5f60304eea7d1147fab83e48568ac82010b68b35e90b2c9808b83e4f97c8f827d597e87562a5fb6009d2cac0ebea314187019baae6b0aa7b5d113cd8501763e05f2c5b8507574bfeea210e435b2b4cc75839cd32ae4d0755e0c2a509c6b75b785941c8507de5a3286cf61d6e681ed4c70a19c0b56c19cfdca7f381c514b21209d05b492f3925bcd981fbfaacbd82b3d526822",
        mac: "03dddb231e54253ee9b404a25ef68213ab6dc88f369e3440f03d5b62ffc6c8ad7ce480f7a170f61a4ddd8f9ac1b276ad",
    },
    TestVector {
        data: "deca016c789111e4833d6c69f79c8121fe64918ee5fded215324c763b95a2e183387a5c6a14708556a8dd5725d9dbb77d0ddab77ed3244e33672c87f52cdf5d742bf3b7a07f5a268f9588efb09b0e82ef7475f62eeb48dd92a20c13863048184d94e4353094ced29de4b2ecc397ca0d962533786adae2956862c1e4067d3248b87c3aa248dac9501321b690c41cda16a9b6c4d1b318000387d81259d20b12b2fc0515ea559f02cbd1cbe6a45a5b30be12effa8f30433d0edf1d3c9fc82aed260d1dc6e53a778f516f2de1666e13bbe1321d211b80b63295856ee300f852cda83561044cdd472c9b5add7fcc0f9f6acb8254d0dc7cc5a80254ab1178662a96fee8bd14a3b591b2c08505a43a9b95a92d06b5730ee60dbd39fa70ed017e6590f952eabd9f36de743824571d1579b34979dc8520cdf1d4a843953ef89f479578e798324108de47e8ec7d9c85cf90a863b9f08794fb35bb342472356c9e36ad4040523fe75fa0afb563448a3f533e3e3a27d5fb20db4fae3d4cbcfbff30b6b0ca872c8de91e0ea3fb14e24f9827882378df3f8f78ce7b76ad74ffa8ff7cbf6cadac2578e34ac97f297892f28c6aa9f926a1a7e5e768a01a752a3532ac878f92ea941931de3e7ad462cefa965de90c0d87e8ee3d509ddb321a57db99149bb7859d3c4611ed28dff95533d85555f0df234b5b1",
        key: "c5158d52be36",
        mac: "8f2414f7a851a4d96b009fa56ff9930d77a8961cc86c6cbdda9f5d47e03bea2b32df77f326b905eac1ce3346c494eabe",
    },
]