// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[cfg(test)]
mod test {

    use crate::clients_data::service::ClientsDataService;

    use crate::clients_data::clients::{GetClientServiceData, UpsertClientServiceData};
    use base::snp::snp_core_types::{ClientIdentityBundle, ClientServiceData, PreKey, PublicKey};
    use base::test_helpers::enable_logger;
    use chrono::prelude::*;
    use crypto::utils::entity_from_pub_key;
    use xactor::Service;

    #[tokio::test]
    async fn test_add_client() {
        enable_logger();
        let clients_data_service = ClientsDataService::from_registry().await.unwrap();
        let client_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);
        let client_id_pub_key = PublicKey {
            key: client_id_key_pair.public.as_ref().to_vec(),
        };

        let client_id_pub = client_id_key_pair.public;
        let client_pre_key_private = x25519_dalek::StaticSecret::new(&mut rand_core::OsRng);
        let client_pre_key_pub_data: x25519_dalek::PublicKey = (&client_pre_key_private).into();
        let pre_key_public: PublicKey = PublicKey {
            key: client_pre_key_pub_data.to_bytes().to_vec(),
        };

        let client_entity = entity_from_pub_key(&client_id_pub_key, "".into());
        let time_stamp = Utc::now().timestamp_nanos() as u64;

        // todo: use current provider bundle and sign the client id bundle here

        let client_bundle = ClientIdentityBundle {
            time_stamp,
            client_id: Some(client_entity),
            address: None,
            provider_bundle: None,
            pre_key: Some(PreKey {
                x2dh_version: "".to_string(),
                key: Some(pre_key_public),
                key_id: 0,
            }),
            one_time_keys: vec![],
            profile_image: None,
            signature: None,
            net_id: 0,
        };

        let client_data = ClientServiceData {
            service_started: 0,
            service_ended: 0,
            client_identity_bundle: Some(client_bundle),
        };

        clients_data_service
            .call(UpsertClientServiceData(client_data))
            .await
            .unwrap()
            .unwrap();

        let res: ClientServiceData = clients_data_service
            .call(GetClientServiceData(client_id_pub))
            .await
            .unwrap()
            .unwrap()
            .unwrap();

        assert!(res.client_identity_bundle.is_some());
        assert_eq!(res.client_identity_bundle.unwrap().time_stamp, time_stamp);
    }
}
