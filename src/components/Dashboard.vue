<template>
     <!-- Demo content -->
  <h2 class="display-4 " style="font-weight:bolder;color:#09123e">Dashboard</h2>

  <div class="separator"></div>
  <div class="row" >
    <div class="col-sm-6">
      <div class="card widget-flat">
          <div class="card-body">
              <div class="float-end">
                  <i class="mdi mdi-account-multiple widget-icon"></i>
              </div>
              <p class="lead mb-0 text-muted">Today</p>
              <h3 class="mt-3 mb-3">3</h3>
            
          </div> <!-- end card-body-->
      </div> <!-- end card-->
  </div>
  <div class="col-sm-6">
    <div class="card widget-flat">
        <div class="card-body">
            <div class="float-end">
                <i class="mdi mdi-account-multiple widget-icon"></i>
            </div>
            <h5 class="text-muted fw-normal mt-0" title="Number of Customers">Male</h5>
            <h3 class="mt-3 mb-3">36,254</h3>
        
        </div> <!-- end card-body-->
    </div> <!-- end card-->
</div>
<div class="col-sm-6">
  <div class="card widget-flat">
      <div class="card-body">
          <div class="float-end">
              <i class="mdi mdi-account-multiple widget-icon"></i>
          </div>
          <h5 class="text-muted fw-normal mt-0" title="Number of Customers">Female</h5>
          <h3 class="mt-3 mb-3">4</h3>
        
      </div> <!-- end card-body-->
  </div> <!-- end card-->
</div>
<div class="col-sm-6">
  <div class="card widget-flat">
      <div class="card-body">
          <div class="float-end">
              <i class="mdi mdi-account-multiple widget-icon"></i>
          </div>
          <h5 class="text-muted fw-normal mt-0" title="Number of Customers">Total</h5>
          <h3 class="mt-3 mb-3">36,254</h3>
      
      </div> <!-- end card-body-->
  </div> <!-- end card-->
</div>

  </div>

</template>
<script>
export default {
    
}
</script>
<style>
.card{
  margin: 0.2rem;
  border-radius: 0;
  border:none;
}
.widget-icon {
  color: #6A75F0A8;
  font-size: 20px;
  background-color: #09123e1a;
  height: 40px;
  width: 40px;
  text-align: center;
  line-height: 40px;
  border-radius: 3px;
  display: inline-block;
}
</style>
