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
  
    background-color: #D5B42E!important;
    color: #fff!important;
    font-weight: bolder!important;
    }

    .class_content{
        min-width: 750px;
        min-height: 400px;
        background-color: rgb(218, 218, 255);
    }
</style>