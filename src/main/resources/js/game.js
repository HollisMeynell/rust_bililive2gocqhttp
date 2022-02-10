let user;
let save;
let map;
let mapdata;
$(()=>{
  $.ajaxSettings.async = false;
  user = localGet('user');
  getSave();
  // console.log(user);
  mapdata = new Object();
  mapdata.EmaxId=0;
  mapdata.EminId=0;
  mapdata.arr = new Array();
  mapdata.E = new Array();

  getMapData(save.user_map,mapdata);
  map = new MAP();

  map.init(mapdata);
  $.ajaxSettings.async = true;
  // map.toEnemy(2,3);
  // map.toPassed(3,4);
  // map.toNext(4,4);
  // map.toStore(3,3);
  map.goEvent((x,y,data)=>{
    map.toPasson(x,y);
  })
  document.oncontextmenu = function() {
    return false;
  }
  $("#map")[0].focus();
  let flag;
  $(document).keydown(e=>{
    if(flag != e.keyCode){
      flag = e.keyCode;
      switch (e.keyCode) {
        case 87:;
        case 38: map.toUp()
          break;
        case 83:;
        case 40: map.toDown()
          break;
        case 65:;
        case 37: map.toLeft()
          break;
        case 68:;
        case 39: map.toRight()
          break;
      }
    }
  })
  $(document).keyup(e=>{
    if(flag == e.keyCode){
      flag = 0;
    }
  })
  // if(user == null) window.location.replace("index.html");
})
function localSave(key, json) {
  localStorage.setItem(key,JSON.stringify(json));
}
function localGet(key) {
  return JSON.parse(localStorage.getItem(key));
}
function localDel(key) {
  localStorage.removeItem(key);
}
function doGet(url,data,funcDo) {
  $.ajax({
    type: "get",
    url: url,
    data: data,
    dataType: "json",
    success: function (e) {
      funcDo(e);
    }
  });
}
function getMapData(mapid,mapdata){
  doGet('../map/findmap.do',{
    'map_id': mapid,
  },(e)=>{
    let gu1 = e.data.enemy_min_id;
    let gu2 = e.data.enemy_max_id;
    mapdata.EminId = gu1;
    mapdata.EmaxId = gu2;
    mapdata.arr[14] = e.data.store_id==0?null:{type: "store",id:e.data.store_id};
    for (let i = gu1; i <= gu2; i++) {
      mapdata.arr[i-gu1] = {
        type:'enemy',
        id:i
      }
    }
    getAllEnemy(mapdata);
  })
}
function getAllEnemy(mapdata){
  for (const mapdatum of mapdata.arr) {
    if(mapdatum != null && mapdatum.type == 'enemy'){
      doGet('../enemy/findenemy.do',{
        'enemy_id':mapdatum.id,
      },(e)=>{
        mapdata.E[e.data.enemy_id] = e.data;
      })
    }
  }
}
function getSave(){
  doGet('../save/read.do',{
    'user_id':user.user_id
  },(e)=>{
    save = e.data;
  })
}
Array.prototype.rand = function () {
  var arr = this
  for (var i = arr.length - 1; i >= 0; i--) {
    var randomIdx = Math.floor(Math.random() * (i + 1))
    var itemAtIdx = arr[randomIdx]
    arr[randomIdx] = arr[i]
    arr[i] = itemAtIdx
  }
  return arr
}
