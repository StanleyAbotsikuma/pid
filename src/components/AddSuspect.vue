<template>
    <h2 class="display-4 " style="font-weight:bolder;color:#09123e">Suspect Form</h2>

    <div class="separator"></div>


    <div class="card" style="width:100%">
        <div class="card-body">

            <form>
                <div id="basicwizard">

                    <ul class="nav nav-pills nav-justified form-wizard-header mb-4" role="tablist">
                        <li class="nav-item" role="presentation">
                            <a href="#basictab1" data-bs-toggle="tab" data-toggle="tab"
                                class="nav-link rounded-0 py-2 active" aria-selected="true" role="tab">
                                <i class="mdi mdi-account-circle font-18 align-middle me-1"></i>
                                <span class="d-none d-sm-inline">Information</span>
                            </a>
                        </li>
                        <li class="nav-item" role="presentation">
                            <a href="#basictab2" data-bs-toggle="tab" data-toggle="tab" class="nav-link rounded-0 py-2"
                                aria-selected="false" role="tab" tabindex="-1">
                                <i class="mdi mdi-face-man-profile font-18 align-middle me-1"></i>
                                <span class="d-none d-sm-inline">Attachments</span>
                            </a>
                        </li>
                        <li class="nav-item" role="presentation">
                            <a href="#basictab3" data-bs-toggle="tab" data-toggle="tab" class="nav-link rounded-0 py-2"
                                aria-selected="false" role="tab" tabindex="-1">
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
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="surname" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-6">
                                        <label for="first_name" class="form-label">FIRST NAME:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="first_name" placeholder="">
                                    </div>
                                </div>

                                <div class="row g-2">
                                    <div class="mb-3 col-md-6">
                                        <label for="middlename" class="form-label">MIDDLE NAME:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="middlename" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-6">
                                        <label for="alias" class="form-label">ALIAS:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="alias"
                                            placeholder="">
                                    </div>
                                </div>



                                <div class="row g-4">
                                    <div class="mb-3 col-md-4">
                                        <label for="date_of_birth" class="form-label">DATE OF BIRTH:</label>
                                        <input type="date" class="form-control form-control-sm shadow-none "
                                            id="date_of_birth" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-2">
                                        <label for="age" class="form-label">AGE:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="age"
                                            placeholder="">
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
                                        <label for="height" class="form-label">HEIGHT<span
                                                class="sublabel">METRE(s)</span>:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="height"
                                            placeholder="">
                                    </div>
                                </div>


                                <div class="row g-3">
                                    <div class="mb-3 col-md-4">
                                        <label for="nose" class="form-label">NOSE:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none " id="nose"
                                            placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-4">
                                        <label for="age" class="form-label">EYES:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="eye"
                                            placeholder="">
                                    </div>

                                    <div class="mb-3 col-md-4">
                                        <label for="masks" class="form-label">MASKS:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="masks"
                                            placeholder="">
                                    </div>
                                </div>
                                <hr style="color:#09123e">
                                <div class="row g-3">
                                    <div class="mb-3 col-md-4">
                                        <label for="place_of_birth" class="form-label">PLACE OF BIRTH:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="place_of_birth" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-4">
                                        <label for="native_of" class="form-label">NATIVE OF:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="native_of" placeholder="">
                                    </div>

                                    <div class="mb-3 col-md-4">
                                        <label for="masks" class="form-label">TRIBE/RACE:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="masks"
                                            placeholder="">
                                    </div>
                                </div>


                                <div class="row g-3">
                                    <div class="mb-3 col-md-4">
                                        <label for="district" class="form-label">DISTRICT:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="district" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-4">
                                        <label for="region" class="form-label">REGION:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="region"
                                            placeholder="">
                                    </div>

                                    <div class="mb-3 col-md-4">
                                        <label for="religion" class="form-label">RELIGION:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="religion" placeholder="">
                                    </div>
                                </div>
                                <hr style="color:#09123e">
                                <div class="row g-2">
                                    <div class="mb-3 col-md-6">
                                        <label for="occupation" class="form-label">OCCUPATION:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="occupation" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-6">
                                        <label for="residential_address" class="form-label">RESIDENTIAL ADDRESS:</label>
                                        <input type="address" class="form-control form-control-sm shadow-none"
                                            id="residential_address" placeholder="1234 Main St">
                                    </div>

                                </div>

                                <div class="row g-2">
                                    <div class="mb-3 col-md-6">
                                        <label for="residence_district" class="form-label">DISTRICT:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="residence_district" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-6">
                                        <label for="residence_region" class="form-label">REGION:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="residence_district" placeholder="">
                                    </div>
                                </div>

                                <div class="row g-2">
                                    <div class="mb-3 col-md-6">
                                        <label for="tel" class="form-label">TEL:</label>
                                        <input type="tel" class="form-control form-control-sm shadow-none " id="tel"
                                            placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-6">
                                        <label for="email" class="form-label">EMAIL:</label>
                                        <input type="email" class="form-control form-control-sm shadow-none" id="email"
                                            placeholder="">
                                    </div>
                                </div>
                                <hr style="color:#09123e">
                                <div class="row g-3">
                                    <div class="mb-3 col-md-4">
                                        <label for="id_type" class="form-label">ID TYPE:</label>
                                        <select id="id_type" class="form-select form-control-sm shadow-none">
                                            <option>Choose</option>
                                            <option>NATIONAL ID</option>
                                            <option>DRIVERS LICENCE</option>
                                            <option>SSNIT</option>
                                            <option>PASSPORT</option>
                                            <option>VOTERS ID</option>
                                            <option>HEALTH INSURANCE</option>

                                        </select>
                                    </div>
                                    <div class="mb-3 col-md-4">
                                        <label for="email" class="form-label">ID NO. :</label>
                                        <input type="email" class="form-control form-control-sm shadow-none" id="email"
                                            placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-4">
                                        <label for="doi" class="form-label">DOI:</label>
                                        <input type="date" class="form-control form-control-sm shadow-none "
                                        id="doi" placeholder="">
                                    </div>
                                </div>
                                <div class="row g-2">
                                    <div class="mb-3 col-md-6">
                                        <label for="poi" class="form-label">POI:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none " id="poi"
                                            placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-6">
                                        <label for="expiry_date" class="form-label">EXPIRY DATE:</label>
                                        <input type="date" class="form-control form-control-sm shadow-none "
                                        id="expiry_date" placeholder="">
                                    </div>
                                </div>
                                <hr style="color:#09123e">
                                <div class="mb-3 col-md-3">
                                    <label for="marital_status" class="form-label">MARITAL STATUS:</label>
                                    <select id="marital_status" class="form-select form-control-sm shadow-none" v-model="marital_status">
                                        <option>Choose</option>
                                        <option>SINGLE</option>
                                        <option>MARRIED</option>
                                        <option>DIVORCED</option>
                                        <option>SEPERATED</option>
                                    </select>
                                </div>
                                <div class="row g-3" >
                                    <div class="mb-3 col-md-4"  v-bind:class = "(marital_status==='SINGLE')?'spouse_false':'spouse_true'">
                                        <label for="name_of_spouse" class="form-label">NAME OF SPOUSE:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="name_of_spouse" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-4"   v-bind:class = "(marital_status==='SINGLE')?'spouse_false':'spouse_true'">
                                        <label for="address_of_spouse" class="form-label" >ADDRESS OF SPOUSE:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="address_of_spouse"
                                            placeholder="">
                                    </div>

                                    <div class="mb-3 col-md-4"   v-bind:class = "(marital_status==='SINGLE')?'spouse_false':'spouse_true'">
                                        <label for="telephone_of_spouse" class="form-label">TELEPHONE OF SPOUSE:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="telephone_of_spouse" placeholder="">
                                    </div>
                                </div>
                                <hr style="color:#09123e">

                                <div class="row g-3">
                                    <div class="mb-3 col-md-4">
                                        <label for="name_of_father" class="form-label">NAME OF FATHER:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="name_of_father" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-4">
                                        <label for="address_of_father" class="form-label">ADDRESS OF FATHER:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="address_of_father"
                                            placeholder="">
                                    </div>

                                    <div class="mb-3 col-md-4">
                                        <label for="telephone_of_father" class="form-label">TELEPHONE OF FATHER:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="religion" placeholder="">
                                    </div>
                                </div>

                                <div class="row g-3">
                                    <div class="mb-3 col-md-4">
                                        <label for="name_of_mother" class="form-label">NAME OF MOTHER:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="name_of_mother" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-4">
                                        <label for="address_of_mother" class="form-label">ADDRESS OF MOTHER:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none" id="region"
                                            placeholder="">
                                    </div>

                                    <div class="mb-3 col-md-4">
                                        <label for="telephone_of_mother" class="form-label">TELEPHONE OF MOTHER:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="telephone_of_mother" placeholder="">
                                    </div>
                                </div>
                                <hr style="color:#09123e">
                                <center><span class="sublabel">FOR FOREIGN NATIONALS ONLY</span></center> 
                                <div class="mb-3">
                                    <label for="aboard_address" class="form-label">ABROAD ADDRESS:</label>
                                    <input type="text" class="form-control form-control-sm shadow-none"
                                        id="aboard_address" placeholder="1234 Main St">
                                </div>
                                <div class="row g-2">
                                    <div class="mb-3 col-md-6">
                                        <label for="state" class="form-label">STATE/REGION:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none "
                                            id="state" placeholder="">
                                    </div>
                                    <div class="mb-3 col-md-6">
                                        <label for="country" class="form-label">COUNTRY:</label>
                                        <input type="text" class="form-control form-control-sm shadow-none"
                                            id="country" placeholder="">
                                    </div>
                                </div>
                                <hr style="color:#09123e">
                                </div>

                            <ul class="list-inline wizard mb-0">
                                <li class="next list-inline-item float-end">
                                    <a href="#" class="btn btn-info">Add More Info <i
                                            class="mdi mdi-arrow-right ms-1"></i></a>
                                </li>
                            </ul>
                        </div>

                        <div class="tab-pane" id="basictab2" role="tabpanel">
                            <div class="class_content">

                            </div>

                            <ul class="pager wizard mb-0 list-inline">
                                <li class="previous list-inline-item disabled">
                                    <button type="button" class="btn btn-light"><i class="mdi mdi-arrow-left me-1"></i>
                                        Back to Account</button>
                                </li>
                                <li class="next list-inline-item float-end">
                                    <button type="button" class="btn btn-info">Add More Info <i
                                            class="mdi mdi-arrow-right ms-1"></i></button>
                                </li>
                            </ul>
                        </div>

                        <div class="tab-pane" id="basictab3" role="tabpanel">
                            <div class="class_content">

                            </div>

                            <ul class="pager wizard mb-0 list-inline mt-1">
                                <li class="previous list-inline-item disabled">
                                    <button type="button" class="btn btn-light"><i class="mdi mdi-arrow-left me-1"></i>
                                        Back to Profile</button>
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
<script>
export default {
    data()
    {
        return{
            marital_status:"SINGLE"
        }
    }
}
</script>
<style scoped>
.card>* {
    border: none !important;
    border-radius: 0 !important;
    padding: 0 !important;
    margin: 0 !important;

}

a {
    color: #09123e !important;
}



.active {

    background-color: #EBC325 !important;
    color: #fff !important;
    font-weight: bolder !important;
}

.class_content {
    min-width: 650px;
    min-height: 400px;
    background-color: #fff !important;
    padding: 1rem;
}

label {
    color: #000;
}


.form-control {
    outline: 0 !important;

}

input,
select {
    outline: 0 !important;
    border-radius: 0%;

}
.spouse_true{
   display: block;
}
.spouse_false{
    display: none;
}
button,.btn {
    width: 100%;
    background-color: #09123e;
    border: none;
    color: white!important;
    border-radius: 0%;
}

button:hover {
    background-color: white;
    color: #09123e;
    border: #09123e solid;
}

.sublabel {
    font-size: small;
    color: burlywood
}

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