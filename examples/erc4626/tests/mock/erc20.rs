#![allow(dead_code)]
#![cfg(feature = "e2e")]
use alloy::{
    primitives::{Address, FixedBytes, U256},
    sol,
};
use e2e::Wallet;
use stylus_sdk::{abi::Bytes, function_selector};


sol! {
    #[allow(missing_docs)]
    // Built with Remix IDE; solc 0.8.24+commit.e11b9ed9
    #[sol(rpc, bytecode="608060405234801561000f575f80fd5b506040516118db3803806118db833981810160405281019061003191906104b7565b828281600390816100429190610743565b5080600490816100529190610743565b5050506100893361006761009160201b60201c565b600a610073919061097a565b8361007e91906109c4565b61009960201b60201c565b505050610aed565b5f6012905090565b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1603610109575f6040517fec442f050000000000000000000000000000000000000000000000000000000081526004016101009190610a44565b60405180910390fd5b61011a5f838361011e60201b60201c565b5050565b5f73ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff160361016e578060025f8282546101629190610a5d565b9250508190555061023c565b5f805f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20549050818110156101f7578381836040517fe450d38c0000000000000000000000000000000000000000000000000000000081526004016101ee93929190610a9f565b60405180910390fd5b8181035f808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2081905550505b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1603610283578060025f82825403925050819055506102cd565b805f808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f82825401925050819055505b8173ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8360405161032a9190610ad4565b60405180910390a3505050565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61039682610350565b810181811067ffffffffffffffff821117156103b5576103b4610360565b5b80604052505050565b5f6103c7610337565b90506103d3828261038d565b919050565b5f67ffffffffffffffff8211156103f2576103f1610360565b5b6103fb82610350565b9050602081019050919050565b8281835e5f83830152505050565b5f610428610423846103d8565b6103be565b9050828152602081018484840111156104445761044361034c565b5b61044f848285610408565b509392505050565b5f82601f83011261046b5761046a610348565b5b815161047b848260208601610416565b91505092915050565b5f819050919050565b61049681610484565b81146104a0575f80fd5b50565b5f815190506104b18161048d565b92915050565b5f805f606084860312156104ce576104cd610340565b5b5f84015167ffffffffffffffff8111156104eb576104ea610344565b5b6104f786828701610457565b935050602084015167ffffffffffffffff81111561051857610517610344565b5b61052486828701610457565b9250506040610535868287016104a3565b9150509250925092565b5f81519050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061058d57607f821691505b6020821081036105a05761059f610549565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026106027fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826105c7565b61060c86836105c7565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61064761064261063d84610484565b610624565b610484565b9050919050565b5f819050919050565b6106608361062d565b61067461066c8261064e565b8484546105d3565b825550505050565b5f90565b61068861067c565b610693818484610657565b505050565b5b818110156106b6576106ab5f82610680565b600181019050610699565b5050565b601f8211156106fb576106cc816105a6565b6106d5846105b8565b810160208510156106e4578190505b6106f86106f0856105b8565b830182610698565b50505b505050565b5f82821c905092915050565b5f61071b5f1984600802610700565b1980831691505092915050565b5f610733838361070c565b9150826002028217905092915050565b61074c8261053f565b67ffffffffffffffff81111561076557610764610360565b5b61076f8254610576565b61077a8282856106ba565b5f60209050601f8311600181146107ab575f8415610799578287015190505b6107a38582610728565b86555061080a565b601f1984166107b9866105a6565b5f5b828110156107e0578489015182556001820191506020850194506020810190506107bb565b868310156107fd57848901516107f9601f89168261070c565b8355505b6001600288020188555050505b505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f8160011c9050919050565b5f808291508390505b6001851115610894578086048111156108705761086f610812565b5b600185161561087f5780820291505b808102905061088d8561083f565b9450610854565b94509492505050565b5f826108ac5760019050610967565b816108b9575f9050610967565b81600181146108cf57600281146108d957610908565b6001915050610967565b60ff8411156108eb576108ea610812565b5b8360020a91508482111561090257610901610812565b5b50610967565b5060208310610133831016604e8410600b841016171561093d5782820a90508381111561093857610937610812565b5b610967565b61094a848484600161084b565b9250905081840481111561096157610960610812565b5b81810290505b9392505050565b5f60ff82169050919050565b5f61098482610484565b915061098f8361096e565b92506109bc7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff848461089d565b905092915050565b5f6109ce82610484565b91506109d983610484565b92508282026109e781610484565b915082820484148315176109fe576109fd610812565b5b5092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610a2e82610a05565b9050919050565b610a3e81610a24565b82525050565b5f602082019050610a575f830184610a35565b92915050565b5f610a6782610484565b9150610a7283610484565b9250828201905080821115610a8a57610a89610812565b5b92915050565b610a9981610484565b82525050565b5f606082019050610ab25f830186610a35565b610abf6020830185610a90565b610acc6040830184610a90565b949350505050565b5f602082019050610ae75f830184610a90565b92915050565b610de180610afa5f395ff3fe608060405234801561000f575f80fd5b5060043610610091575f3560e01c8063313ce56711610064578063313ce5671461013157806370a082311461014f57806395d89b411461017f578063a9059cbb1461019d578063dd62ed3e146101cd57610091565b806306fdde0314610095578063095ea7b3146100b357806318160ddd146100e357806323b872dd14610101575b5f80fd5b61009d6101fd565b6040516100aa9190610a5a565b60405180910390f35b6100cd60048036038101906100c89190610b0b565b61028d565b6040516100da9190610b63565b60405180910390f35b6100eb6102af565b6040516100f89190610b8b565b60405180910390f35b61011b60048036038101906101169190610ba4565b6102b8565b6040516101289190610b63565b60405180910390f35b6101396102e6565b6040516101469190610c0f565b60405180910390f35b61016960048036038101906101649190610c28565b6102ee565b6040516101769190610b8b565b60405180910390f35b610187610333565b6040516101949190610a5a565b60405180910390f35b6101b760048036038101906101b29190610b0b565b6103c3565b6040516101c49190610b63565b60405180910390f35b6101e760048036038101906101e29190610c53565b6103e5565b6040516101f49190610b8b565b60405180910390f35b60606003805461020c90610cbe565b80601f016020809104026020016040519081016040528092919081815260200182805461023890610cbe565b80156102835780601f1061025a57610100808354040283529160200191610283565b820191905f5260205f20905b81548152906001019060200180831161026657829003601f168201915b5050505050905090565b5f80610297610467565b90506102a481858561046e565b600191505092915050565b5f600254905090565b5f806102c2610467565b90506102cf858285610480565b6102da858585610512565b60019150509392505050565b5f6012905090565b5f805f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20549050919050565b60606004805461034290610cbe565b80601f016020809104026020016040519081016040528092919081815260200182805461036e90610cbe565b80156103b95780601f10610390576101008083540402835291602001916103b9565b820191905f5260205f20905b81548152906001019060200180831161039c57829003601f168201915b5050505050905090565b5f806103cd610467565b90506103da818585610512565b600191505092915050565b5f60015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2054905092915050565b5f33905090565b61047b8383836001610602565b505050565b5f61048b84846103e5565b90507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811461050c57818110156104fd578281836040517ffb8f41b20000000000000000000000000000000000000000000000000000000081526004016104f493929190610cfd565b60405180910390fd5b61050b84848484035f610602565b5b50505050565b5f73ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff1603610582575f6040517f96c6fd1e0000000000000000000000000000000000000000000000000000000081526004016105799190610d32565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16036105f2575f6040517fec442f050000000000000000000000000000000000000000000000000000000081526004016105e99190610d32565b60405180910390fd5b6105fd8383836107d1565b505050565b5f73ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff1603610672575f6040517fe602df050000000000000000000000000000000000000000000000000000000081526004016106699190610d32565b60405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff16036106e2575f6040517f94280d620000000000000000000000000000000000000000000000000000000081526004016106d99190610d32565b60405180910390fd5b8160015f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208190555080156107cb578273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516107c29190610b8b565b60405180910390a35b50505050565b5f73ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff1603610821578060025f8282546108159190610d78565b925050819055506108ef565b5f805f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20549050818110156108aa578381836040517fe450d38c0000000000000000000000000000000000000000000000000000000081526004016108a193929190610cfd565b60405180910390fd5b8181035f808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2081905550505b5f73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1603610936578060025f8282540392505081905550610980565b805f808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f82825401925050819055505b8173ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef836040516109dd9190610b8b565b60405180910390a3505050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f610a2c826109ea565b610a3681856109f4565b9350610a46818560208601610a04565b610a4f81610a12565b840191505092915050565b5f6020820190508181035f830152610a728184610a22565b905092915050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610aa782610a7e565b9050919050565b610ab781610a9d565b8114610ac1575f80fd5b50565b5f81359050610ad281610aae565b92915050565b5f819050919050565b610aea81610ad8565b8114610af4575f80fd5b50565b5f81359050610b0581610ae1565b92915050565b5f8060408385031215610b2157610b20610a7a565b5b5f610b2e85828601610ac4565b9250506020610b3f85828601610af7565b9150509250929050565b5f8115159050919050565b610b5d81610b49565b82525050565b5f602082019050610b765f830184610b54565b92915050565b610b8581610ad8565b82525050565b5f602082019050610b9e5f830184610b7c565b92915050565b5f805f60608486031215610bbb57610bba610a7a565b5b5f610bc886828701610ac4565b9350506020610bd986828701610ac4565b9250506040610bea86828701610af7565b9150509250925092565b5f60ff82169050919050565b610c0981610bf4565b82525050565b5f602082019050610c225f830184610c00565b92915050565b5f60208284031215610c3d57610c3c610a7a565b5b5f610c4a84828501610ac4565b91505092915050565b5f8060408385031215610c6957610c68610a7a565b5b5f610c7685828601610ac4565b9250506020610c8785828601610ac4565b9150509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680610cd557607f821691505b602082108103610ce857610ce7610c91565b5b50919050565b610cf781610a9d565b82525050565b5f606082019050610d105f830186610cee565b610d1d6020830185610b7c565b610d2a6040830184610b7c565b949350505050565b5f602082019050610d455f830184610cee565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610d8282610ad8565b9150610d8d83610ad8565b9250828201905080821115610da557610da4610d4b565b5b9291505056fea2646970667358221220230760f8d917e582c42a859ebdc96dd7de02dd61f36ea2e2688baa8be80b0f4664736f6c634300081a0033")]
    contract ERC20Mock is ERC20 {
        constructor(string memory name_, string memory symbol_) {
            _name = name_;
            _symbol = symbol_;
        }
    }
}

pub async fn deploy(
    wallet: &Wallet,
    error: ERC1155ReceiverMock::RevertType,
) -> eyre::Result<Address> {
    let contract =
        ERC20Mock::deploy(wallet, REC_RETVAL, BAT_RETVAL, error)
            .await?;
    Ok(*contract.address())
}