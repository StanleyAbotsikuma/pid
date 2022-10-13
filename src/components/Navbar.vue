<template>
    <!-- Vertical navbar -->
<div class="vertical-nav bg-white" id="sidebar" >
    <div class="py-4 px-3 mb-4 ">
      <div class="media d-flex align-items-center"><img src="@/assets/logo.png" width="65" class=" img-thumbnail " style="border:none">
        <div class="media-body">
          <h4 class="m-0">Police Intellegence</h4>
          <p class="font-weight-light text-muted mb-0">Suspect Database</p>
        </div>
      </div>
    </div>
    <div class="py-4" style="background-color:#09123e;height:100%">
    <p class="text-gray font-weight-bold text-uppercase px-3 small pb-4 mb-0">Menu</p>
  
    <ul class="nav flex-column  mb-0">
      <li class="nav-item " v-bind:class = "(p_route==='Dashboard')?'active':''">
        <a href="#" class="nav-link text-dark font-italic" @click="routePage('Dashboard')">
                  <i class="fa fa-th-large mr-3 text-primary fa-fw"></i>
                  Dashboard
              </a>
      </li>
      <li class="nav-item" v-bind:class = "(p_route==='AddSuspect')?'active':''">
        <a href="#" class="nav-link text-dark font-italic" @click="routePage('AddSuspect')">
                  <i class="fa fa-address-card mr-3 text-primary fa-fw"></i>
                  Add Suspect
              </a>
      </li>
      <li class="nav-item" v-bind:class = "(p_route==='SuspectList')?'active':''">
        <a href="#" class="nav-link text-dark font-italic" @click="routePage('SuspectList')">
                  <i class="fa fa-cubes mr-3 text-primary fa-fw"></i>
                  Suspect List
              </a>
      </li>
      <li class="nav-item" v-bind:class = "(p_route==='Reports')?'active':''">
        <a href="#" class="nav-link text-dark font-italic" @click="routePage('Reports')">
                  <i class="fa fa-picture-o mr-3 text-primary fa-fw"></i>
                  Reports
              </a>
      </li>
    </ul>
  <hr>
  <div style="bottom:0px;width:100%;position:absolute;height:200px">
    <ul class="nav flex-columnmb-0">
      <li class="nav-item">
        <a href="#" class="nav-link text-dark font-italic" @click="signin">
                  <i class="fa fa-area-chart mr-3 text-primary fa-fw"></i>
                  Sign out
              </a>
      </li>
      <li class="nav-item">
        <a href="#" class="nav-link text-dark font-italic" style="color: #015de7!important; font-weight: 500;" @click="exit">
                  <i class="fa fa-bar-chart mr-3 text-primary fa-fw"></i>
                 Exit
              </a>
      </li>
      
    </ul>
    </div>
    </div>
  </div>
 
</template>


<style scoped>
a {
  color:aliceblue!important
}

</style>
<script>
import { confirm } from '@tauri-apps/api/dialog';
export default {
  data()
  {
    return{
      p_route: "Dashboard",

    }
  },
methods: {
    routePage(name)
    {
      this.p_route =name;
    this.$emit("pageChange", name)
      // this.aclas = !this.aclas;

    },
    async signin()
    {
      let text = "Do you want to sign-out?";
  if (await confirm(text,{title:"GHANA POLICE INTELLEGENCE",type:"warning"}) == true) {
     this.$router.push("/");
   
  } else {
    text = "You canceled!";
  }
    },
    exit()
    {
      let text = "Do you want to exit this system?";
  if (confirm(text) == true) {
    text = "You pressed OK!";
  } else {
    text = "You canceled!";
  }
    }
    
  },
}
</script>

<style scoped>
.active{
  
background-color: #987e19;
}
</style>