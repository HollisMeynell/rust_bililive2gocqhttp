function MAP() {
  this.length = 15;
  this.map = [];
  for(let x = 1; x <= 4; x++){
    this.map[x] = [];
    for(let y = 1; y <= 4; y++){
      this.map[x][y] = $(`#map [row='${x}'][column='${y}']`);
    }
  }
  this.goEvent = function(fun){
    $("#map").on("mousedown","div",e=>{
      let x = parseInt($(e.target).attr("row"));
      let y = parseInt($(e.target).attr("column"));
      let by = this.getBody();
      if(e.which == 1){
        if(1 == (Math.abs(x-by[0])+Math.abs(y-by[1]))){
          fun(x,y,this.getEvent(x,y));
        }
      }else if(e.which == 4){
        fun(x,y,this.getEvent(x,y));
      };
    });
    this.toUp = function () {
      let by = this.getBody();
      fun(by[0]>1?by[0]-1:by[0],by[1],this.getEvent(by[0]>1?by[0]-1:by[0],by[1]));
    }
    this.toDown = function () {
      let by = this.getBody();
      fun(by[0]<4?by[0]+1:by[0],by[1],this.getEvent(by[0]<4?by[0]+1:by[0],by[1]));
    }
    this.toLeft = function () {
      let by = this.getBody();
      fun(by[0],by[1]>1?by[1]-1:by[1],this.getEvent(by[0],by[1]>1?by[1]-1:by[1]));
    }
    this.toRight = function () {
      let by = this.getBody();
      fun(by[0],by[1]<4?by[1]+1:by[1],this.getEvent(by[0],by[1]<4?by[1]+1:by[1]));
    }
  };

  this.toPassed = function (x,y) {
    this.map[x][y].removeClass();
    this.map[x][y].addClass('map_passed');
  }
  this.toPasson = function (x,y) {
    let p = $("#map .map_passon");
    if(p.length != 0) {
      let c = this.getBody();
      this.toPassed(c[0],c[1]);
    }
    this.map[x][y].removeClass();
    this.map[x][y].addClass('map_passon');
  }
  this.toPassout = function (x,y) {
    this.map[x][y].removeClass();
    this.map[x][y].addClass('map_passout');
  }
  this.toEnemy = function (x,y) {
    this.map[x][y].removeClass();
    this.map[x][y].addClass('map_enemy');
  }
  this.toStore = function (x,y) {
    this.map[x][y].removeClass();
    this.map[x][y].addClass('map_store');
  }
  this.toNext = function (x,y) {
    this.map[x][y].removeClass();
    this.map[x][y].addClass('map_next');
  }


  this.toPassedN = function (n) {
    let x = 1 + parseInt((n-1)/4);
    let y = 1 + (n-1)%4;
    this.toPassed(x,y);
  }
  this.toPassonN = function (n) {
    let x = 1 + parseInt((n-1)/4);
    let y = 1 + (n-1)%4;
    this.toPasson(x,y);
  }
  this.toPassoutN = function (n) {
    let x = 1 + parseInt((n-1)/4);
    let y = 1 + (n-1)%4;
    this.toPassout(x,y);
  }
  this.toEnemyN = function (n) {
    let x = 1 + parseInt((n-1)/4);
    let y = 1 + (n-1)%4;
    this.toEnemy(x,y);
  }
  this.toStoreN = function (n) {
    let x = 1 + parseInt((n-1)/4);
    let y = 1 + (n-1)%4;
    this.toStore(x,y);
  }
  this.toNextN = function (n) {
    let x = 1 + parseInt((n-1)/4);
    let y = 1 + (n-1)%4;
    this.toNext(x,y);
  }
  this.getBody = function(){
    let p = $("#map .map_passon");
    if(p.length != 0){
      let x = parseInt(p.attr("row"));
      let y = parseInt(p.attr("column"));
      return [x,y];
    }
    return [0,0];
  }
  this.setEvent = function(x,y,data){
    this.map[x][y].Idata = data;
  }
  this.setEvent = function(index,data){
    let x = 1 + parseInt((index-1)/4);
    let y = 1 + (index-1)%4;
    this.map[x][y].Idata = data;
  }
  this.getEvent = function (x,y) {
    return this.map[x][y].Idata;
  }
  this.toLeft = function () {
    console.log('l')
  }
  this.toRight = function () {
    console.log('r')
  }
  this.toUp = function () {
    console.log('u')
  }
  this.toDown = function () {
    console.log('d')
  }
  this.init = function (mapdata) {
    /***
     * mapdata.
     */
    this.toPasson(1,1);
    for (let i = 2; i <= 15; i++) {
      this.toPassoutN(i);
    }
    this.toNextN(16);
  }
}
