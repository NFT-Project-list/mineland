#!/bin/bash
set -e

if [ ${#NEAR_ID} -eq 0 ]
then
 read -p "Enter NEAR_ID: " NEAR_ID
fi

CONTRACT_ID=$(<neardev/dev-account)
#
echo "--- Init Contracts:"
near call nft-stone.$CONTRACT_ID new_default_meta '{"owner_id":"'$NEAR_ID'"}' --accountId $NEAR_ID
near call nft-mine.$CONTRACT_ID new_default_meta '{"owner_id":"'$NEAR_ID'"}' --accountId $NEAR_ID
near call ft.$CONTRACT_ID new_default_meta '{"owner_id":"'$CONTRACT_ID'", "total_supply": "1000000000000000000000000000000000"}' --accountId $NEAR_ID

echo "--- Seed Contracts Data:"
near call $CONTRACT_ID add_collection '{"title": "Diamond", "image": "bafkreihvyvwqw6i5wfsvcaa33b7lbwqedkxo37jkaraowrkffcnvgkp74a", "stone_images": ["bafkreihvyvwqw6i5wfsvcaa33b7lbwqedkxo37jkaraowrkffcnvgkp74a", "bafkreif6s3x3ptcn47zxx4zkpazbrwphoc2cpv2vnwil7twsv3zmcns62a", "bafkreifasykvgid35csgbqmam6ywtg3hmgwed6gg3fm532cjl4mv5fz27m","bafkreiel7u7q4fxle4u6ztbkn3v224jc64axiqzl22wipvicqtbdt2btrm","bafkreifw2rdcnd3itqhhtoqsqvvjwtcprkwnu6dzokmg3rb4ailpjqufpe"]}' --accountId $NEAR_ID
near call $CONTRACT_ID add_collection '{"title": "Igneous", "image": "bafkreiarev46fysxe2reltfzhd47j7pkw4kfkmql6nsniavkg36mo6bhhe", "stone_images": ["bafkreiarev46fysxe2reltfzhd47j7pkw4kfkmql6nsniavkg36mo6bhhe", "bafkreibysle5scntjhgcn3ejwxqiemtjwyqg6fnd4qyqg7vqmpl6kzdc34", "bafkreiehejazb5ky5oyb2k4s6ru7zaagx5zpdcvwfdz3iiwpiukwqxfdqy", "bafkreieliiumdzzjsljdxai5bti5zony66el7cx7rdltig3e2flws7dxui", "bafkreidnkl4tvp6fiwl3blwcypzransx24c3w7h7b36pyleb27uq3hh6hu", "bafkreiey2mvvsmqlhcdndalpqvwcnkx2qkqtf6illyjlr2ozm2anynot4u", "bafkreicchliwm7sfrv4zqi2pf3zgtjljbjvxvv77o6vfhb26q53v3z77aq"]}' --accountId $NEAR_ID

echo "--- Mint Free Mine:"
LAND_MINT_RESPONSE=$(near call $CONTRACT_ID mint_mine_nft --accountId $NEAR_ID --deposit 0.01 --gas 60000000000000)
echo $LAND_MINT_RESPONSE
LAND_ID=$(awk -F'token_id: |,' '{print $3}' <<< $LAND_MINT_RESPONSE)
LAND_ID=${LAND_ID:1}
LAND_ID=${LAND_ID%?}

echo "--- Mint Free Stone:"
ZOMBIE_MINT_RESPONSE=$(near call $CONTRACT_ID mint_free_stone_nft '{"mine_id": "'$LAND_ID'"}' --accountId $NEAR_ID --deposit 0.01 --gas 50000000000000)
echo $ZOMBIE_MINT_RESPONSE

ZOMBIE_ID=$(awk -F'token_id: |,' '{print $3}' <<< $ZOMBIE_MINT_RESPONSE)
ZOMBIE_ID=${ZOMBIE_ID:1}
ZOMBIE_ID=${ZOMBIE_ID%?}

echo "--- Place Mine to the Market:"
near call $CONTRACT_ID publish_mines_on_market '{"token_price_list":{"'$LAND_ID'":"1000000000000000000000000"}}' --accountId $NEAR_ID --deposit 0.000000000000000000000001

echo "--- Place Stone to the Market:"
near call $CONTRACT_ID publish_stones_on_market '{"token_price_list":{"'$ZOMBIE_ID'":"500000000000000000000000"}}' --accountId $NEAR_ID --deposit 0.000000000000000000000001


#for (( i=1; i <= 10; i++ ))
#do
#echo "--- Mint Free Stone:"
#ZOMBIE_MINT_RESPONSE=$(near call $CONTRACT_ID mint_free_stone_nft '{"mine_id": "l-1-178"}' --accountId $NEAR_ID --deposit 0.06 --gas 200000000000000)
#echo $ZOMBIE_MINT_RESPONSE
#done
