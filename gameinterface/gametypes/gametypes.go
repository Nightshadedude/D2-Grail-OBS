package gametypes

//VADDR holds address data
type VADDR uint16

//MS_WCHAR_T holds wide char data from the DLL
type MS_WCHAR_T uint8

//DWORD hold double word data
type DWORD uint16

//WORD holds word data
type WORD uint8

//BYTE holds byte data
type BYTE uint8

//LONG holds long data
type LONG uint32

//POINT holds X, Y coordinates
type POINT struct {
	X int
	Y int
}

//Automap_layer is automap later data
type Automap_layer struct {
	Layer_no DWORD // 0x00
	F_saved DWORD 
	Floors *Automap_cell 
	Walls *Automap_cell
	Objects *Automap_cell
	Extras *Automap_cell
	Next *Automap_layer // 0x18
	//need to figure out the recursive reference to the same struct
}	

//Automap_layer2 is the second later ot automap data
type Automap_layer2 struct {
	_1 [2]DWORD
	Layer_no DWORD // 0x08
}

// Level - updated
type Level struct {
	_1 [4]DWORD 
	Room2_first Room2 // 0x10
	_2 [2]DWORD
	X DWORD // 0x1C
	Y DWORD // 0x20
	Size_y DWORD // 0x24
	Size_x DWORD // 0x28
	_3 [0x180]BYTE
	Next Level // 0x1AC
	_4 DWORD
	Misc *Act_misc // 0x1B4
	_5 [3]DWORD
	Seed [2]DWORD // 0x1C4
	_6 DWORD
	Level_no DWORD // 0x1D0
}

// Room2 - updated
type Room2 struct {
	_1 [2]DWORD
	Room2_near Room2 // 0x08
	_2 [2]DWORD
	Seed [2]DWORD // 0x14
	Prev *Room2 // 0x1C
	_3 DWORD
	Next *Room2 // 0x24
	_4 DWORD
	N_rooms_near DWORD // 0x2C
	Room1 *Room1 // 0x30
	X DWORD // 0x34
	Y DWORD // 0x38
	Size_x DWORD // 0x3C
	Size_y DWORD // 0x40
	_5 [2]DWORD
	Room_tiles *Room_tiles // 0x4C
	_6 [2]DWORD 
	Level *Level // 0x58
	Preset *Preset_unit // 0x5C
	_7 [0x88]BYTE
	Other *Room2 // 0xE8
}

// Room1 - updated
type Room1 struct {
  Room1s_near **Room1 // 0x00
  _1 [3]DWORD
  Room2 *Room2 // 0x10
  _2 [3]DWORD
  Coll_map VADDR  // 0x20
  N_rooms_near DWORD // 0x24
  _3 DWORD
  Act *Act // 0x2C
  _4 [0x1C]BYTE
  Base_x DWORD // 0x4C
  Base_y DWORD // 0x50
  Size_x DWORD // 0x54
  Size_y DWORD // 0x58
  R_x DWORD // 0x5C
  R_y DWORD // 0x60
  R_size_x DWORD // 0x64
  R_size_y DWORD // 0x68
  Seed [2]DWORD // 0x6C
  Unit_first *Unit_any // 0x74
  _5 DWORD
  Next *Room1; // 0x7C
}

// Act_misc - updated
type Act_misc struct {
  _1 [0x94]BYTE
  Staff_tomb_level DWORD // 0x94
  _2 [0x3D4]BYTE
  Act *Act // 0x46C
  _3 [3]DWORD
  Level_first *Level // 0x47C
}

// Act - updated
type Act struct {
  _1 [4]DWORD 
  Room1 *Room1 // 0x10
  Act_no DWORD // 0x14
  _2 [12]DWORD
  Misc *Act_misc // 0x48
}

//Unit_any - port
type Unit_any struct {
  Type DWORD
  Txtfile_no DWORD
  _1 DWORD
  Id DWORD
  Mode DWORD
  //union
  Monster_data *Monster_data
  Player_data *Player_data
  Item_data *Item_data
  Object_data VADDR
  //end union
  Act_no DWORD 
  Act *Act;
  Seed [2]DWORD
  _2 DWORD
  //union
  Path *Path;
  Item_path VADDR 
  Object_path VADDR
  //end union
  _3 [5]DWORD
  Gfx_frame DWORD
  Frame_remain DWORD
  Frame_rate WORD
  _4 WORD
  Gfx_unk *BYTE
  Gfx_info *DWORD
  _5 DWORD
  Stats VADDR
  Inventory VADDR
  Light VADDR 
  _6 [9]DWORD 
  X WORD 
  Y WORD
  _7 DWORD
  Owner_type DWORD
  Owner_id DWORD
  _8 [2]DWORD
  Oh_msg VADDR
  Info VADDR
  _9 [6]DWORD
  Flags DWORD
  Flags_2 DWORD
  _10 [5]DWORD
  Changed_next *Unit_any
  Room_next *Unit_any
  List_next *Unit_any
}

//Path - Port
type Path struct {
  Offset_x WORD
  X WORD
  Offset_y WORD
  Y WORD
  _1 [2]DWORD
  Target_x WORD
  Target_y WORD
  _2 [2]DWORD
  Room1 *Room1
  Room_unk *Room1
  _3 [3]DWORD
  Unit *Unit_any
  Flags DWORD
  _4 DWORD
  Type DWORD 
  Prev_type DWORD 
  Unit_size DWORD 
  _5 [4]DWORD 
  Target_unit *Unit_any
  Target_type DWORD
  Target_id DWORD 
  Direction BYTE 
}

// Preset_unit - updated
type Preset_unit struct {
  _1 DWORD 
  Txt_file_no DWORD  // 0x04
  X DWORD // 0x08
  Next *Preset_unit // 0x0C
  _2 DWORD 
  Type DWORD // 0x14
  Y DWORD // 0x18
}


// HERE
// updated
struct room_tile
{
  room2 *room2; // 0x00
  room_tile *next; // 0x04
  DWORD _1[2];
  DWORD *num; // 0x10
};

struct monster_data
{
  BYTE _1[22];
  struct
  {
    BYTE unk :1;
    BYTE normal :1;
    BYTE champ :1;
    BYTE boss :1;
    BYTE minion :1;
  };
  BYTE _2[5];
  BYTE enchants[9];
  WORD unique_no;
  DWORD _5;
  struct
  {
    ms_wchar_t name[28];
  };
};

struct object_txt
{
  char s_name[0x40];
  ms_wchar_t ws_name[0x40];
  BYTE _1[4];
  BYTE selectable;
  BYTE _2[0x87];
  BYTE orientation;
  BYTE _3[0x19];
  BYTE subclass;
  BYTE _4[0x11];
  BYTE parm;
  BYTE _5[0x39];
  BYTE populate_fn;
  BYTE operate_fn;
  BYTE _6[8];
  DWORD automap;
};

struct Automap_cell
{
  DWORD f_saved;
  WORD cell_no;
  WORD pixel_x;
  WORD pixel_y;
  WORD weight;
  Automap_cell *less;
  Automap_cell *more;
};

struct item_data
{
  DWORD quality;
  DWORD _1[2];
  DWORD item_flags;
  DWORD _2[2];
  DWORD flags;
  DWORD _3[3];
  DWORD quality2;
  DWORD level;
  DWORD _4[2];
  WORD prefix;
  WORD _5[2];
  WORD suffix;
  DWORD _6;
  BYTE body_location;
  BYTE item_location;
  BYTE _7;
  WORD _8;
  DWORD _9[4];
  VADDR owner_inventory;
  DWORD _10;
  unit_any *inv_next;
  BYTE _11;
  BYTE node_page;
  WORD _12;
  DWORD _13[6];
  unit_any *owner;
};

// updated
struct game_info
{
  BYTE _1[0x1B];
  char game_name[0x18]; // 0x1B
  char game_ip[0x56]; // 0x33
  /*
   * haven't checked anything below here, but I
   * guess those offsets haven't changed as well
   */
  char acc_name[0x30]; // 0x89
  char char_name[0x18]; // 0xB9
  char realm_name[0x18]; // 0xD1
  BYTE _2[0x158];
  char game_pass[0x18]; // 0x241
};

struct player_data
{
  char name[0x10];
  VADDR quest_normal;
  VADDR quest_nightmare;
  VADDR quest_hell;
  VADDR waypoint_normal;
  VADDR waypoint_nightmare;
  VADDR waypoint_hell;
};

#elif defined _VERSION_1_12

/* structs for version 1.12a (from D2BS) */

struct Automap_layer
  {
    DWORD layer_no;
    DWORD f_saved;
    Automap_cell *floors;
    Automap_cell *walls;
    Automap_cell *objects;
    Automap_cell *extras;
    Automap_layer *next;
  };

struct Automap_layer2
  {
    DWORD _1[2];
    DWORD layer_no;
  };

struct level
  {
    BYTE _1[0x50];
    DWORD seed[2];
    DWORD _2;
    level *next;
    DWORD _3;
    act_misc *misc;
    DWORD _4;
    DWORD x;
    DWORD y;
    DWORD size_x;
    DWORD size_y;
    DWORD _5[6];
    DWORD level_no;
    DWORD _6[0x61];
    room2 *room2_first;
  };

struct room2
  {
    level *level;
    DWORD _1;
    DWORD n_rooms_near;
    room_tile *room_tiles;
    room2 **room2_near;
    DWORD _3[6];
    DWORD x;
    DWORD y;
    DWORD size_x;
    DWORD size_y;
    VADDR type2info;
    DWORD _4[0x20];
    DWORD preset_type;
    preset_unit *preset;
    DWORD _5[0x3];
    room2 *next;
    room1 *room1;
    DWORD seed[2];
  };

struct room1
  {
    room1 **rooms1_near;
    DWORD _1[2];
    DWORD seed[2];
    DWORD _2;
    DWORD base_x;
    DWORD base_y;
    DWORD size_x;
    DWORD size_y;
    DWORD _3[0x4];
    room1 *next;
    DWORD _4;
    unit_any *unit_first;
    DWORD _5[3];
    VADDR coll_map;
    DWORD _6[0x7];
    room2 *room2;
    DWORD _7;
    DWORD n_rooms_near;
  };

struct act_misc
  {
    DWORD _1;
    act *act;
    DWORD _2[238];
    DWORD staff_tomb_level;
    DWORD _3[43];
    level *level_first;
  };

struct act
  {
    BYTE _1[0x34];
    room1 *room1;
    act_misc *misc;
    DWORD _2[2];
    DWORD act_no;
  };

struct unit_any
  {
    DWORD type;
    DWORD txtfile_no;
    DWORD _1;
    DWORD id;
    DWORD mode;
    union
      {
        monster_data *monster_data;
        player_data *player_data;
        item_data *item_data;
        VADDR object_data;
      };
    DWORD act_no;
    act *act;
    DWORD seed[2];
    DWORD _2;
    union
      {
        path *path;
        VADDR item_path;
        VADDR object_path;
      };
    DWORD _3[5];
    DWORD gfx_frame;
    DWORD frame_remain;
    WORD frame_rate;
    WORD _4;
    BYTE *gfx_unk;
    DWORD *gfx_info;
    DWORD _5;
    VADDR stats;
    VADDR inventory;
    VADDR light;
    DWORD _6[9];
    WORD x;
    WORD y;
    DWORD _7;
    DWORD owner_type;
    DWORD owner_id;
    DWORD _8[2];
    VADDR oh_msg;
    VADDR info;
    DWORD _9[6];
    DWORD flags;
    DWORD flags_2;
    DWORD _10[5];
    unit_any *changed_next;
    unit_any *room_next;
    unit_any *list_next;
  };

struct path
  {
    WORD offset_x;
    WORD x;
    WORD offset_y;
    WORD y;
    DWORD _1[2];
    WORD target_x;
    WORD target_y;
    DWORD _2[2];
    room1 *room1;
    room1 *room_unk;
    DWORD _3[3];
    unit_any *unit;
    DWORD flags;
    DWORD _4;
    DWORD type;
    DWORD prev_type;
    DWORD unit_size;
    DWORD _5[4];
    unit_any *target_unit;
    DWORD Target_type;
    DWORD Target_id;
    BYTE Direction;
  };

struct Automap_cell
  {
    DWORD f_saved;
    WORD cell_no;
    WORD pixel_x;
    WORD pixel_y;
    WORD weight;
    Automap_cell *less;
    Automap_cell *more;
  };

struct preset_unit
  {
    DWORD txt_file_no;
    DWORD _1[2];
    DWORD x;
    DWORD _2;
    DWORD y;
    preset_unit *next;
    DWORD type;
  };

struct object_txt
  {
    char s_name[0x40];
    ms_wchar_t ws_name[0x40];
    BYTE _1[4];
    BYTE selectable;
    BYTE _2[0x87];
    BYTE orientation;
    BYTE _3[0x19];
    BYTE subclass;
    BYTE _4[0x11];
    BYTE parm;
    BYTE _5[0x39];
    BYTE populate_fn;
    BYTE operate_fn;
    BYTE _6[8];
    DWORD automap;
  };

struct room_tile
  {
    DWORD *num;
    room2 *room2;
    DWORD _1[2];
    room_tile *next;
  };

struct monster_data
  {
    BYTE _1[22];
    struct
      {
        BYTE unk:1;
        BYTE normal:1;
        BYTE champ:1;
        BYTE boss:1;
        BYTE minion:1;
      };
    BYTE _2[5];
    BYTE enchants[9];
    WORD unique_no;
    DWORD _5;
    struct
      {
        ms_wchar_t name[28];
      };
  };

struct player_data
  {
    char name[0x10];
    VADDR quest_normal;
    VADDR quest_nightmare;
    VADDR quest_hell;
    VADDR waypoint_normal;
    VADDR waypoint_nightmare;
    VADDR waypoint_hell;
  };

struct item_data
  {
    DWORD quality;
    DWORD _1[2];
    DWORD item_flags;
    DWORD _2[2];
    DWORD flags;
    DWORD _3[3];
    DWORD quality2;
    DWORD level;
    DWORD _4[2];
    WORD prefix;
    WORD _5[2];
    WORD suffix;
    DWORD _6;
    BYTE body_location;
    BYTE item_location;
    BYTE _7;
    WORD _8;
    DWORD _9[4];
    VADDR owner_inventory;
    DWORD _10;
    unit_any *inv_next;
    BYTE _11;
    BYTE node_page;
    WORD _12;
    DWORD _13[6];
    unit_any *owner;
  };

struct game_info
  {
    DWORD _1[6];
    WORD _2;
    char game_name[0x18];
    char game_ip[0x56];
    char acc_name[0x30];
    char char_name[0x18];
    char realm_name[0x18];
    BYTE _3[0x157];
    char game_pass[0x18];
  };
