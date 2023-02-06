import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { LoginComponent } from './login.component';
import {PageOTPComponent} from "../page-otp/page-otp.component";

const routes: Routes = [
  { path: '', component: LoginComponent, children:[
      {path: 'otp', component: PageOTPComponent}
    ] }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class LoginRoutingModule { }
