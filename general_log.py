import io
import datetime
import binascii
import json
import struct

bots_names_list = [
    "ajay",
    "bobby",
    "carver",
    "casimiro",
    "drayke",
    "jun",
    "khan",
    "laraya",
    "maaz",
    "morrissey",
    "natalya",
    "no-1",
    "odalis",
    "rhymer",
    "rin",
    "shannon",
]

locations = {
    "207260926": {
        "location_id": 1,
        "location_name": "barcelona_oval",
    },
    "1762682682": {
        "location_id": 2,
        "location_name": "barcelona_bowl",
    },
    "496308235": {
        "location_id": 3,
        "location_name": "brighton",
    },
    "1890962255": {
        "location_id": 4,
        "location_name": "barcelona_gracia",
    },
    "1059041770": {
        "location_id": 5,
        "location_name": "hackney",
    },
    "2049317438": {
        "location_id": 6,
        "location_name": "hollywood_hills",
    },
    "2115846046": {
        "location_id": 7,
        "location_name": "amboy",
    },
    "2712304153": {
        "location_id": 8,
        "location_name": "la_downtown",
    },
    "2910297654": {
        "location_id": 9,
        "location_name": "amboy_bowl",
    },
    "3509833163": {
        "location_id": 10,
        "location_name": "la_docks",
    },
    "1248814327": {
        "location_id": 11,
        "location_name": "la_downtown_highrise",
    },
    "1576813183": {
        "location_id": 12,
        "location_name": "la_downtown_figureof8",
    },
    "751655752": {
        "location_id": 13,
        "location_name": "la_river",
    },
    "1832430229": {
        "location_id": 14,
        "location_name": "mount_haruna",
    },
    "1891155750": {
        "location_id": 15,
        "location_name": "ny_dumbo",
    },
    "1426934948": {
        "location_id": 16,
        "location_name": "sanfran_russian_hill",
    },
    "1471873234": {
        "location_id": 17,
        "location_name": "sanfran_sausalito",
    },
    "651065683": {
        "location_id": 18,
        "location_name": "tokyo_shutoko",
    },

}

def sync_make_game_dict(game_binary_log:bytes) -> dict:
    #file_reader = io.BytesIO(open('logData_race_haruna', 'rb').read())
    file_reader = io.BytesIO(game_binary_log)
    with open('./db_converter/maps.json') as json_file:
        map_data = json.load(json_file)

    with open('./db_converter/cars.json') as json_file:
        car_data = json.load(json_file)

    def get_map_name(map_hex_id: bytes) -> str:

        map_int_id = int().from_bytes(map_hex_id, 'little')
        #print(map_int_id)
        if str(map_int_id) in map_data:
            if map_data[str(map_int_id)]['map_name'] != None:
                return map_data[str(map_int_id)]['map_name']
            else:
                return map_data[str(map_int_id)]['map_internal_name']

    def get_car_name(car_hex_id: bytes) -> str:

        car_int_id = int().from_bytes(car_hex_id, 'little')
        #print(car_int_id)
        if str(car_int_id) in car_data:
            if car_data[str(car_int_id)]['car_name'] != None:
                return car_data[str(car_int_id)]['car_name']

    def get_mod_name(mod_id:int)-> str:
        mods_data = {
            1: "iron_fist",
            2: "jump_the_gun",
            3: "front_runner",
            4: "drifter",
            5: "titanium_armour",
            6: "showy_flourish",
            7: "stable_frame",
            8: "battering_ram",
            9: "decoy_drop",
            10: "magnetic_field",
            11: "scrambler",
            12: "splash_damage",
            13: "shielding_efficiency",
            14: "safety_net",
            15: "shielded_boosters",
            16: "shielded_bay",
            17: "ecm",
            18: "vampiric_wreck",
            19: "bribe",
            20: "fan_favourite",
            21: "laser_sight",
            22: "advanced_radar",
            23: "silent_running",
            24: "last_gasp",
            25: "mastermine",
            26: "additionalblindfireshots",
            27: "additionalshockdomes",
            28: "doubleshield",
            29: "fansnitrogift",
            30: "nitrorift",
            31: "repairbonus",
            32: "shotgunblindfire",
            33: "shuntdamage",
        }

        if mod_id+1 in mods_data:
            return mods_data[mod_id+1]
        elif mod_id == 4294967295:
            return "None"
        else:
            return "Unknown"

    def get_game_type_name(game_type_id:int) -> str:

        game_types = {
            24392580: "community_event",
            2288498497: "team_racing",
            2366728894: "team_motor_mash",
            3468808035: "motor_mash",
            1647796835: "powered_up_racing",
            1786934394: "skirmish_racing",
            1527855959: "world_tour",
            3131866246: "hardcore_racing",
            4114270737: "driving_school",
            0: "custom_game"
        }

        if game_type_id in game_types:
            return game_types[game_type_id]
        else:
            return "Unknown"

    def get_location(location_id:int):

        if str(location_id) in locations:
            return locations[str(location_id)]
        else:
            return {'location_id':0,'location_name':"unknown"}

    game_dict = {}

    game_type_id_bytes = file_reader.read(4)
    game_type_id = int().from_bytes(game_type_id_bytes,'little')

    game_dict['game_type_hex'] = game_type_id_bytes.hex()
    game_dict['game_type_id'] = game_type_id
    game_dict['game_type_name'] = get_game_type_name(game_type_id)

    file_reader.seek(16)
    location_id_bytes = int().from_bytes(file_reader.read(4),'little')
    location_data = get_location(location_id_bytes)

    game_dict['location_id'] = location_data['location_id']
    game_dict['location_name'] = location_data['location_name']

    map_id_bytes = file_reader.read(4)

    map_name = get_map_name(map_id_bytes)
    game_dict['map_name'] = map_name
    #print(map_name)
    total_racers = int().from_bytes(file_reader.read(4),'little')
    game_dict['number_of_racers'] = total_racers
    #print(f"Number of racers: {total_racers}")

    laps = int().from_bytes(file_reader.read(4),'little')
    #print(f"Number of laps: {laps}")
    game_dict['laps'] = laps

    file_reader.read(4)

    racers_info = {}

    for i in range(0, total_racers):

        temp_dict = {}

        player_name = file_reader.read(64).decode('utf-16').encode('ascii', 'ignore').replace(b'\x00',b'').decode()
        #print(player_name)
        temp_dict['player_name'] = player_name

        if temp_dict['player_name'].lower() in bots_names_list:
            temp_dict['is_bot'] = True
        else:
            temp_dict['is_bot'] = False

        player_hash = file_reader.read(8).hex()
        temp_dict['player_hash'] = player_hash
        #print(player_id)
        test_list = []
        text_hex_list = []
        for j in range(0,23):
            #print(f"READER POS: {file_reader.tell()}")
            if j == 0:
                hex = file_reader.read(4)
                text_hex_list.append(hex.hex())
                test = struct.unpack('f', hex)[0]
                #print(test)
                test_list.append(test)
            elif j == 1:
                hex = file_reader.read(4)
                text_hex_list.append(hex.hex())
                test = struct.unpack('f', hex)[0]
                #print(test)
                temp_dict['traveled_distance'] = test
            elif j == 2:
                mod1 = int().from_bytes(file_reader.read(4), 'little')
                #print(f"LEVEL {level}")
                temp_dict['mod_1_id'] = 0 if mod1 == 4294967295 else mod1 + 1
                temp_dict['mod_1_name'] = get_mod_name(mod1)
            elif j == 3:
                mod2 = int().from_bytes(file_reader.read(4), 'little')
                #print(f"LEVEL {level}")
                temp_dict['mod_2_id'] = 0 if mod2 == 4294967295 else mod2 + 1
                temp_dict['mod_2_name'] = get_mod_name(mod2)
            elif j == 4:
                mod3 = int().from_bytes(file_reader.read(4), 'little')
                #print(f"LEVEL {level}")
                temp_dict['mod_3_id'] = 0 if mod3 == 4294967295 else mod3 + 1
                temp_dict['mod_3_name'] = get_mod_name(mod3)
            elif j == 5:
                level = int().from_bytes(file_reader.read(4), 'little')
                #print(f"LEVEL {level}")
                temp_dict['player_level'] = level
            elif j == 6:
                legend = int().from_bytes(file_reader.read(4), 'little')
                #print(f"LEVEL {level}")
                temp_dict['player_legend'] = legend
            elif j == 7:
                #print("CAR ID")
                car_name = get_car_name(file_reader.read(4))
                temp_dict['player_car_name'] = car_name
                #print(car_name)
            elif j == 9:
                #print("CAR ID")
                total_fans = int().from_bytes(file_reader.read(4), 'little')
                temp_dict['total_fans'] = total_fans
                #print(car_name)
            else:

                hex = file_reader.read(4)
                text_hex_list.append(hex.hex())
                #print(hex.hex())
                test = int().from_bytes(hex, 'little')
                #print(test)
                test_list.append(test)
        #print(test_list)
        #print(text_hex_list)
        #print(file_reader.tell())
        temp_dict['starting_pos'] = int().from_bytes(file_reader.read(1),'little') + 1
        placed = int().from_bytes(file_reader.read(1),'little')
        temp_dict['finish_pos'] = placed
        #print(f"PLACE: {placed}")


        temp_dict['final_state_id'] = int().from_bytes(file_reader.read(2), 'little')

        def final_state_str(final_state_id:int)->str:
            states_dict = {
                2:"Finished",
                3:"DNF"
            }

            if final_state_id in states_dict:
                return states_dict[final_state_id]
            else:
                return "unknown"

        temp_dict['final_state'] = final_state_str(temp_dict['final_state_id'])

        racers_info[placed] = temp_dict.copy()




        #print('+++++++++')


    game_dict['racers_info'] = racers_info.copy()

    #print(game_dict)
    #print(json.dumps(game_dict))
    return game_dict

if __name__ == "__main__":
    file_reader = open('logData_with_bots', 'rb').read()
    print(json.dumps(sync_make_game_dict(file_reader)))
