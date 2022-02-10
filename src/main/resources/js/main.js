let user;
function toRegister() {
  $("#register").removeClass("shanchu");
  $(".center[id!=register]").removeClass("fuxian");
  $("#register").addClass("fuxian");
  $(".center[id!=register]").addClass("shanchu");
}
function toBack(){
  $("#login").removeClass("shanchu");
  $(".center[id!=login]").removeClass("fuxian");
  $("#login").addClass("fuxian");
  $(".center[id!=login]").addClass("shanchu");
}
  function toRole(){
  $("#role").removeClass("shanchu");
  $(".center[id!=role]").removeClass("fuxian");
  $("#role").addClass("fuxian");
  $(".center[id!=role]").addClass("shanchu");
  $(".user_role[ad='left']").css("animation","to_left 300ms forwards");
  $(".user_role[ad='center']").css("animation","to_center 300ms forwards");
  $(".user_role[ad='right']").css("animation","to_right 300ms forwards");
}
function toLogin(){
  let name = $("input[name='username']").val();
  let pwd = $("input[name='password']").val();
  if(name == '' || pwd==''){
    alert("用户名或密码不能为空");
    return false;
  }else{
    $.ajax({
    type: "get",
    url: "../user/login.do",
    data: {
      'user_name':name,
      'user_password':pwd
    },
    dataType: "json",
    success: function (e) {
      if(e.success){
        $.cookie('uid',e.message);
        user = e.data;
        localSave('user',user);
        if(user.user_role == 0){
          toRole();
        }else {
          window.location.replace("game.html");
        }
      }
      else{
        alert(e.message);
      }
    }
  });
  }
}
function toRoleIn() {
      if($(".user_role.chouse").length==1){
        doGet('../user/chooserole.do',{
          'user_id':user.user_id,
          'user_role':$(".user_role.chouse").attr('roleid')
        },(e)=>{
          user.user_role = parseInt($(".user_role.chouse").attr('roleid'));
          localSave('user',user);
          doGet('../save/write.do',{
            'user_id':user.user_id,
            'user_max_hp': 500,
            'user_hp':500,
            'user_attack':100,
            'user_defense':50,
            'user_exp':0,
            'user_level':1,
            'user_weapon':0,
            'user_map':1,
            'user_money':0
          },(e)=>{
            console.log(e)
          })
          window.location.replace("game.html");
        })
      }else {
        alert("选择一个职业");
      }
}
function toSubmit(){
  let apd = $("input[name='r_password']").val();
  let bpd = $("input[name='c_password']").val();
  let name = $("input[name='r_username']").val();
  let zz = /[^\w\d]/
  if(name == '' || zz.test(name)){
    alert("用户名为空或包含特殊字符\n只允许英文加数字");
  }else if(apd != bpd){
    alert("密码不一致");
  }else if($("input[name='r_username']").attr('flag') == 'false'){
    alert("用户名未通过");
  }else{
    $.ajax({
      type: "get",
      url: "../user/register.do",
      data: {
        'user_name':name,
        'user_password':apd
      },
      dataType: "json",
      success: function (e) {
        if(e.success){
          $("input[name='username']").val(e.data.user_name);
          $("input[name='password']").val(apd);
          toBack();
        }
      }
    });
  }
}


$(()=>{
  $(".user_role").click(function () {
    $(".user_role").removeClass('chouse');
    $(this).addClass('chouse');
    let a = $(this).attr("ad")=='left'?"剑士":$(this).attr("ad")=='center'?"斗士":"圣甲士";
    $(".role_info").text(a);
  });

  $("input[name='r_username']").blur(function () {
    doGet('../user/userexist.do',{
      'user_name':$("input[name='r_username']").val(),
    },(e)=>{
      if(e.success){
        $("input[name='r_username']").attr('flag',true);
      }else {
        alert(e.message);
      }
    })
  })

  document.getElementById("bgm").play();
});
function localSave(key, json) {
  localStorage.setItem(key,JSON.stringify(json));
}
function localGet(key) {
  return JSON.parse(localStorage.getItem(key));
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

if(false){
  doGet('../map/findmap.do',{
    'map_id':'1',
  },(e)=>{
    console.log(e)
  })
  doGet('../prop/findprop.do',{
    'prop_id':'1',
  },(e)=>{
    console.log(e)
  })
  doGet('../store/findstore.do',{
    'store_id':'1',
  },(e)=>{
    console.log(e)
  })
  doGet('../enemy/findenemy.do',{
    'enemy_id':'1',
  },(e)=>{
    console.log(e)
  })
  doGet('../role/inti.do',{
    'user_role':'3',
  },(e)=>{
    console.log(e)
  })
  doGet('../role/allrole.do',{
    'all':'5',
  },(e)=>{
    console.log(e)
  })
  doGet('../user/login.do',{
    'user_name':'admin',
    'user_password':'123123'
  },(e)=>{
    console.log(e)
  })
//判定重复用户名
  doGet('../user/userexist.do',{
    'user_name':'admin',
  },(e)=>{
    console.log(e)
  })
  doGet('../user/chooserole.do',{
    'user_id':'840dcf23-2918-455a-8d25-21700ec935d2',
    'user_role':'3'
  },(e)=>{
    console.log(e)
  })
  doGet('../user/win.do',{
    'user_id':'840dcf23-2918-455a-8d25-21700ec935d2',
    'user_win':'3',
  },(e)=>{
    console.log(e)
  })
  doGet('../save/read.do',{
    'user_id':'840dcf23-2918-455a-8d25-21700ec935d2',
  },(e)=>{
    console.log(e)
  })
  doGet('../save/write.do',{
    'user_id':'840dcf23-2918-455a-8d25-21700ec935d2',
    'user_max_hp':'2000',
    'user_hp':'1444',
    'user_attack':'150',
    'user_defense':'450',
    'user_exp':'808',
    'user_level':'5',
    'user_weapon':'0',
    'user_map':'2',
    'user_money':'800'
  },(e)=>{
    console.log(e)
  })

}
