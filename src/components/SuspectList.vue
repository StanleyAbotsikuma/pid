<template>
    <h2 class="display-4 " style="font-weight:bolder;color:#09123e">Suspect List</h2>

    <div class="separator"></div>
<div style="background-color:white;width:770px">
    <div class="row">
        <div class="col-md-12">
            <div class="table-wrap">
                <table class="table">
                  <thead>
                    <tr>
                        
                      <th>#ID</th>
                      <th>Full Name</th>
                      <th>Telephone</th>
                      <th>Username</th>
                      <th>Status</th>
                      <th>Actions</th>
                    
                     
                    </tr>
                  </thead>
                  <tbody>
                    <tr class="alert" role="alert">
                        <td>
                            1
                        </td>
                      <td class="d-flex align-items-center">
                          <img class="img" src="../assets/logo.png">
                          <div class="pl-3 email">
                              <span>markotto@email.com</span>
                              <span>markotto@email.com</span>
                              <span>Added: 01/03/2020</span>
                          </div>
                      </td>
                      <td>Markotto89</td>
                      <td>Markotto89</td>
                      <td>Markotto89</td>
                      <td><a href="#" class="btn">   <font-awesome-icon icon="fas fa-sign-out" />View</a></td>
                  
                    </tr>
             
                    
                  </tbody>
                </table>
            </div>
        </div>
    </div>
    </div>
                                        
</template>


<style scoped>

  
  .img {
    background-size: cover;
    background-repeat: no-repeat;
    background-position: center center; }
  

   
  .table {
   
    width: 100%!important;
    -webkit-box-shadow: 0px 5px 12px -12px rgba(0, 0, 0, 0.29);
    -moz-box-shadow: 0px 5px 12px -12px rgba(0, 0, 0, 0.29);
    box-shadow: 0px 5px 12px -12px rgba(0, 0, 0, 0.29); }
    .table thead th {
      border: none;
      padding: 12px;
      font-size: 13px;
      font-weight: 500;
      color: gray; }
    .table thead tr {
      background: #fff;
      border-bottom: 4px solid #eceffa; }
    .table tbody tr {
      margin-bottom: 10px;
      border-bottom: 4px solid #f8f9fd;
    overflow-wrap: break-word; }
      .table tbody tr:last-child {
        border-bottom: 0; }
    .table tbody th, .table tbody td {
      border: none;
      padding:10px;
      font-size: 14px;
      background: #fff;
      vertical-align: middle; }
    .table tbody td.status span {
      position: relative;
      border-radius: 30px;
      padding: 4px 10px 4px 25px; }
      .table tbody td.status span:after {
        position: absolute;
        top: 9px;
        left: 10px;
        width: 10px;
        height: 10px;
        content: '';
        border-radius: 50%; }
    .table tbody td.status .active {
      background: #cff6dd;
      color: #1fa750; }
      .table tbody td.status .active:after {
        background: #23bd5a; }
    .table tbody td.status .waiting {
      background: #fdf5dd;
      color: #cfa00c; }
      .table tbody td.status .waiting:after {
        background: #f2be1d; }
    .table tbody td .img {
      width: 50px;
      height: 50px;
      border-radius: 50%; }
    .table tbody td .email span {
      display: block; }
      .table tbody td .email span:last-child {
        font-size: 12px;
        color: rgba(0, 0, 0, 0.3); }
    .table tbody td .close span {
      font-size: 12px;
      color: #dc3545; }
  
  .checkbox-wrap {
    display: block;
    position: relative;
    cursor: pointer;
    font-size: 16px;
    font-weight: 500;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none; }
  
  /* Hide the browser's default checkbox */
  .checkbox-wrap input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
    height: 0;
    width: 0; }
  
  /* Create a custom checkbox */
  .checkmark {
    position: absolute;
    top: 0;
    left: 0; }
  
  /* Create the checkmark/indicator (hidden when not checked) */
  .checkmark:after {
    content: "\f0c8";
    font-family: "FontAwesome";
    position: absolute;
    color: rgba(0, 0, 0, 0.1);
    font-size: 20px;
    margin-top: -14px;
    -webkit-transition: 0.3s;
    -o-transition: 0.3s;
    transition: 0.3s; }
    @media (prefers-reduced-motion: reduce) {
      .checkmark:after {
        -webkit-transition: none;
        -o-transition: none;
        transition: none; } }
  
  /* Show the checkmark when checked */
  .checkbox-wrap input:checked ~ .checkmark:after {
    display: block;
    content: "\f14a";
    font-family: "FontAwesome";
    color: rgba(0, 0, 0, 0.2); }
  
  /* Style the checkmark/indicator */
  .checkbox-primary {
    color: #40bfc1; }
    .checkbox-primary input:checked ~ .checkmark:after {
      color: #40bfc1; }
  
</style>
<style>

</style>