#!/bin/zsh
print "正在生成chain spec1..."
./target/release/parachain-collator build-spec --disable-default-bootnode > rococo-local-parachain-plain.json
print "正在生成chain spec2..."
./target/release/parachain-collator build-spec --chain rococo-local-parachain-plain.json --raw --disable-default-bootnode > rococo-local-parachain-1000-raw.json
print "正在生成genesis..."
./target/release/parachain-collator export-genesis-state --chain rococo-local-parachain-1000-raw.json > para-1000-genesis
print "正在生成wasm..."
./target/release/parachain-collator export-genesis-wasm --chain rococo-local-parachain-1000-raw.json > para-1000-wasm    
print "完成！"
