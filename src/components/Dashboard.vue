<template>
     <!-- Demo content -->
  <h2 class="display-4 " style="font-weight:bolder;color:#09123e">Dashboard</h2>

  <div class="separator"></div>
  <div class="row class_content">
    <div class="col-sm-6">
      <div class="card widget-flat">
          <div class="card-body">
              <div class="float-end">
                
                <font-awesome-icon icon="fas fa-calendar " class="widget-icon" />
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
              <font-awesome-icon icon="fas fa-male " class="widget-icon" />
            </div>
            <p class="lead mb-0 text-muted">Males</p>
            <h3 class="mt-3 mb-3">36,254</h3>
        
        </div> <!-- end card-body-->
    </div> <!-- end card-->
</div>
<div class="col-sm-6">
  <div class="card widget-flat">
      <div class="card-body">
          <div class="float-end">
            <font-awesome-icon icon="fas fa-female " class="widget-icon" />

          </div>
          <p class="lead mb-0 text-muted">Females</p>
          <h3 class="mt-3 mb-3">4</h3>
        
      </div> <!-- end card-body-->
  </div> <!-- end card-->
</div>
<div class="col-sm-6">
  <div class="card widget-flat">
      <div class="card-body">
          <div class="float-end">
            <font-awesome-icon icon="fas fa-users" class="widget-icon" />
          </div>
          <p class="lead mb-0 text-muted">Total</p>
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
  color: rgba(9, 18, 62, 0.762);
 
  height: 30px;
  width: 30px;
  text-align: center;
  line-height: 40px;
  border-radius: 3px;
  display: inline-block;
  vertical-align:middle;
  
}

.class_content{
  min-width: 650px;
  min-height: fit-content;


}
</style>
