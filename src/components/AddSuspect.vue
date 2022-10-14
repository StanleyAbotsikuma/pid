<template>
    <h2 class="display-4 " style="font-weight:bolder;color:#09123e">Suspect Form</h2>

    <div class="separator"></div>

   
        <div class="card" style="width:100%">
            <div class="card-body">

                <form>
                    <div id="basicwizard">

                        <ul class="nav nav-pills nav-justified form-wizard-header mb-4" role="tablist">
                            <li class="nav-item" role="presentation">
                                <a href="#basictab1" data-bs-toggle="tab" data-toggle="tab" class="nav-link rounded-0 py-2 active" aria-selected="true" role="tab"> 
                                    <i class="mdi mdi-account-circle font-18 align-middle me-1"></i>
                                    <span class="d-none d-sm-inline">Information</span>
                                </a>
                            </li>
                            <li class="nav-item" role="presentation">
                                <a href="#basictab2" data-bs-toggle="tab" data-toggle="tab" class="nav-link rounded-0 py-2" aria-selected="false" role="tab" tabindex="-1">
                                    <i class="mdi mdi-face-man-profile font-18 align-middle me-1"></i>
                                    <span class="d-none d-sm-inline">Attachments</span>
                                </a>
                            </li>
                            <li class="nav-item" role="presentation">
                                <a href="#basictab3" data-bs-toggle="tab" data-toggle="tab" class="nav-link rounded-0 py-2" aria-selected="false" role="tab" tabindex="-1">
                                    <i class="mdi mdi-checkbox-marked-circle-outline font-18 align-middle me-1"></i>
                                    <span class="d-none d-sm-inline ">Finish</span>
                                </a>
                            </li>
                        </ul>

                        <div class="tab-content b-0 mb-0">
                            <div class="tab-pane active show" id="basictab1" role="tabpanel">
                                <div class="class_content">

                                    <div class="row g-2">
                                        <div class="mb-3 col-md-6">
                                            <label for="lastname" class="form-label">SURNAME:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none " id="surname" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-6">
                                            <label for="first_name" class="form-label">FIRST NAME:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="first_name" placeholder="">
                                        </div>
                                    </div>

                                    <div class="row g-2">
                                        <div class="mb-3 col-md-6">
                                            <label for="middlename" class="form-label">MIDDLE NAME:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none " id="middlename" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-6">
                                            <label for="alias" class="form-label">ALIAS:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="alias" placeholder="">
                                        </div>
                                    </div>



                                    <div class="row g-4">
                                        <div class="mb-3 col-md-4">
                                            <label for="date_of_birth" class="form-label">DATE OF BIRTH:</label>
                                            <input type="date" class="form-control form-control-sm shadow-none " id="date_of_birth" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-2">
                                            <label for="age" class="form-label">AGE:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="age" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-3">
                                            <label for="sex" class="form-label">SEX:</label>
                                            <select id="sex" class="form-select form-control-sm shadow-none">
                                                <option>Choose</option>
                                                <option>MALE</option>
                                                <option>FEMALE</option>
                                              
                                            </select>
                                        </div>
                                        <div class="mb-3 col-md-3">
                                            <label for="height" class="form-label">HEIGHT<span class="sublabel">METRE(s)</span>:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="height" placeholder="">
                                        </div>
                                    </div>
                                    
                                    
                                    <div class="row g-3">
                                        <div class="mb-3 col-md-4">
                                            <label for="nose" class="form-label">NOSE:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none " id="nose" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-4">
                                            <label for="age" class="form-label">EYES:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="eye" placeholder="">
                                        </div>
                                       
                                        <div class="mb-3 col-md-4">
                                            <label for="masks" class="form-label">MASKS:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="masks" placeholder="">
                                        </div>
                                    </div>
                                    <hr style="color:#09123e">
                                    <div class="row g-3">
                                        <div class="mb-3 col-md-4">
                                            <label for="place_of_birth" class="form-label">PLACE OF BIRTH:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none " id="place_of_birth" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-4">
                                            <label for="native_of" class="form-label">NATIVE OF:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="native_of" placeholder="">
                                        </div>
                                       
                                        <div class="mb-3 col-md-4">
                                            <label for="masks" class="form-label">TRIBE/RACE:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="masks" placeholder="">
                                        </div>
                                    </div>

                                   
                                    <div class="row g-3">
                                        <div class="mb-3 col-md-4">
                                            <label for="district" class="form-label">DISTRICT:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none " id="district" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-4">
                                            <label for="region" class="form-label">REGION:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="region" placeholder="">
                                        </div>
                                       
                                        <div class="mb-3 col-md-4">
                                            <label for="religion" class="form-label">RELIGION:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="religion" placeholder="">
                                        </div>
                                    </div>
<hr style="color:#09123e">
                                    <div class="row g-2">
                                        <div class="mb-3 col-md-6">
                                            <label for="occupation" class="form-label">OCCUPATION:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none " id="occupation" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-6">
                                            <label for="residential_address" class="form-label">RESIDENTIAL ADDRESS:</label>
                                            <input type="address" class="form-control form-control-sm shadow-none" id="residential_address" placeholder="1234 Main St">
                                           </div>
                                       
                                    </div>

                                    <div class="row g-2">
                                        <div class="mb-3 col-md-6">
                                            <label for="residence_district" class="form-label">DISTRICT:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none " id="residence_district" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-6">
                                            <label for="residence_region" class="form-label">REGION:</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="residence_district" placeholder="">
                                        </div>
                                    </div>

                                    <div class="row g-2">
                                        <div class="mb-3 col-md-6">
                                            <label for="tel" class="form-label">TEL:</label>
                                            <input type="tel" class="form-control form-control-sm shadow-none " id="tel" placeholder="">
                                        </div>
                                        <div class="mb-3 col-md-6">
                                            <label for="email" class="form-label">EMAIL:</label>
                                            <input type="email" class="form-control form-control-sm shadow-none" id="email" placeholder="">
                                        </div>
                                    </div>
                                    <hr style="color:#09123e">

                                    <div class="mb-3">
                                        <label for="inputAddress" class="form-label">Address</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="inputAddress" placeholder="1234 Main St">
                                    </div>
                                    
                                    <div class="mb-3">
                                        <label for="inputAddress2" class="form-label">Address 2</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="inputAddress2" placeholder="Apartment, studio, or floor">
                                    </div>
                                                
                                    <div class="row g-2">
                                        <div class="mb-3 col-md-6">
                                            <label for="inputCity" class="form-label">City</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="inputCity">
                                        </div>
                                        <div class="mb-3 col-md-4">
                                            <label for="inputState" class="form-label">State</label>
                                            <select id="inputState" class="form-select">
                                                <option>Choose</option>
                                                <option>MALE</option>
                                                <option>FEMALE</option>
                                                <option>Option 3</option>
                                            </select>
                                        </div>
                                        <div class="mb-3 col-md-2">
                                            <label for="inputZip" class="form-label">Zip</label>
                                            <input type="text" class="form-control form-control-sm shadow-none" id="inputZip">
                                        </div>
                                    </div>
                                
                                    <div class="mb-2">
                                        <div class="form-check">
                                            <input type="checkbox" class="form-check-input" id="customCheck11">
                                            <label class="form-check-label" for="customCheck11">Check this custom checkbox</label>
                                        </div>
                                    </div>
                                </div>

                                <ul class="list-inline wizard mb-0">
                                    <li class="next list-inline-item float-end">
                                        <a href="javascript:void(0);" class="btn btn-info">Add More Info <i class="mdi mdi-arrow-right ms-1"></i></a>
                                    </li>
                                </ul>
                            </div>

                            <div class="tab-pane" id="basictab2" role="tabpanel">
                                <div class="class_content">

                                </div>

                                <ul class="pager wizard mb-0 list-inline">
                                    <li class="previous list-inline-item disabled">
                                        <button type="button" class="btn btn-light"><i class="mdi mdi-arrow-left me-1"></i> Back to Account</button>
                                    </li>
                                    <li class="next list-inline-item float-end">
                                        <button type="button" class="btn btn-info">Add More Info <i class="mdi mdi-arrow-right ms-1"></i></button>
                                    </li>
                                </ul>
                            </div>

                            <div class="tab-pane" id="basictab3" role="tabpanel">
                                <div class="class_content">

                                </div>

                                <ul class="pager wizard mb-0 list-inline mt-1">
                                    <li class="previous list-inline-item disabled">
                                        <button type="button" class="btn btn-light"><i class="mdi mdi-arrow-left me-1"></i> Back to Profile</button>
                                    </li>
                                    <li class="next list-inline-item float-end">
                                        <button type="button" class="btn btn-info">Submit</button>
                                    </li>
                                </ul>
                            </div>

                        </div> <!-- tab-content -->
                    </div> <!-- end #basicwizard-->
                </form>

            </div> <!-- end card-body -->
        </div>
 
</template>

<style scoped>
.card>*
{
    border:none!important;
    border-radius:0!important;
    padding: 0!important;
    margin:0!important;

}
a{
    color: #09123e!important;
}



.active{
  
    background-color:#EBC325!important;
    color: #fff!important;
    font-weight: bolder!important;
    }

    .class_content{
        min-width: 650px;
        min-height: 400px;
        background-color:  #fff!important;
        padding: 1rem;
    }

    label{
        color: #000;
    }
    

    .form-control {
        outline: 0!important;
      
    }
    input,select{
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
    .sublabel{font-size:small;color:burlywood}
      textarea:focus, 
textarea.form-control:focus, 
input.form-control:focus, 
input[type=text]:focus, 
input[type=password]:focus, 
input[type=email]:focus, 
input[type=number]:focus, 
[type=text].form-control:focus, 
[type=password].form-control:focus, 
[type=email].form-control:focus, 
[type=tel].form-control:focus, 
[contenteditable].form-control:focus {
  box-shadow: inset 0 -1px 0 #ddd;
}
</style>