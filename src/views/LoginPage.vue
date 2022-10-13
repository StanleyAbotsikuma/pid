<template>
    <div class="text-center">
       
        <img src="@/assets/logo.png" alt="" srcset="" style="width:120px"/>
        <br><br>
        <h2>Sign In</h2>
      
      </div>
   
      <br>
    <div class="container" >

        <form>
            
   
            <div class="form-outline">
                <label class="form-label" for="form2Example1">Username</label>
              <input type="email" id="form2Example1" class="form-control shadow-none" />
            
            </div>
          
            <!-- Password input -->
            <div class="form-outline ">
                <label class="form-label" for="form2Example2">Password</label>
              <input type="password" id="form2Example2" class="form-control shadow-none" />
            </div>
          
            <!-- 2 column grid layout for inline styling -->
            <div class="row">
              <div class="col d-flex justify-content-center">
                <!-- Checkbox -->
                
              </div>
          
          
            </div>
          <br>
            <button type="button" class="btn btn-primary btn-block mb-4" @click="login">Sign in</button>
          
            
           
          </form> 
    </div> 
   
</template>
<style scoped>
*{
    outline: 0;
    color: #09123e;
}
input{
    outline: 0!important;
    border-radius: 0%;
    
  }
 

  button{
    width: 100%;
    background-color: #09123e;
    border: none;
  }
  button:hover{
    background-color: white;
    color: #09123e;
    border: #09123e solid;
  }

</style>

<script>
import { invoke } from '@tauri-apps/api';
export default {
    methods:{
         async login(e)
        {
e.preventDefault();
await invoke("login", { username: "a",password:"1" }).then((e)=>{
  if(e =="success")
  {
     this.$router.push("home");

  }
});
// ;
        }
    }
}

</script>
