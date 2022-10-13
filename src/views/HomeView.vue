<script setup>
import Navbar from '../components/Navbar.vue';
import Dashboard from '../components/Dashboard.vue';
import AddSuspect from '../components/AddSuspect.vue';
import SuspectListVue from '../components/SuspectList.vue';
import Reports from '../components/Reports.vue';
</script>

<script>
export default {
  components:[Navbar,Dashboard,AddSuspect,SuspectListVue,Reports],
 data(){
  return{
    pageRoute:"Dashboard",
  }
 },
  methods: {
    getPageChange(data){
    this.pageRoute=data;
    // alert(this.pageRoute);
    }
  },

}
</script>
<template>
  <Navbar v-on:pageChange="getPageChange"/>
  

   
<!-- Page content holder -->

<div class="page-content p-5 body" id="content" >
<Dashboard v-if="pageRoute === 'Dashboard'" />
<AddSuspect v-else-if="pageRoute === 'AddSuspect'"/>
<SuspectListVue v-else-if="pageRoute === 'SuspectList'"/>
<Reports v-else-if="pageRoute === 'Reports'"/>
<div v-else class="muted center">
 <center>
  <h1 class="lead"> Page Not Found</h1> 
  <h2 >404</h2></center>
</div>

</div>
<!-- End demo content -->


</template>

