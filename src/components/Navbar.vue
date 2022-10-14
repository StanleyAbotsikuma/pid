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
               
                  Dashboard
                  
              </a>
      </li>
      <li class="nav-item" v-bind:class = "(p_route==='AddSuspect')?'active':''">
        <a href="#" class="nav-link text-dark font-italic" @click="routePage('AddSuspect')">
               
                  Add Suspect
              </a>
      </li>
      <li class="nav-item" v-bind:class = "(p_route==='SuspectList')?'active':''">
        <a href="#" class="nav-link text-dark font-italic" @click="routePage('SuspectList')">
              
                  Suspect List
              </a>
      </li>
      <li class="nav-item" v-bind:class = "(p_route==='Reports')?'active':''">
        <a href="#" class="nav-link text-dark font-italic" @click="routePage('Reports')">
              
                  Reports
              </a>
      </li>
    </ul>
  <hr>
  <div style="bottom:0px;width:100%;position:absolute;height:193px">
    <ul class="nav ">
      
      <li class="nav-item">
        <a href="#" class="nav-link text-dark font-italic btn" style="color: #015de7!important; font-weight: 500;    background-color: #AA0202C3;
        color: #ffffff !important;border-radius:0" @click="exit">
          <font-awesome-icon icon="fas fa-power-off" />              
              </a>
      </li>
      <li class="nav-item">
        <a href="#" class="nav-link text-dark font-italic btn" @click="signin" style="color: #fff!important; font-weight: 500;">
          <font-awesome-icon icon="fas fa-sign-out" />
                 
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
  
background-color: #C0A01F;
}
</style>