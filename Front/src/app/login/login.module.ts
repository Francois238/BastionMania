import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';

import {LoginRoutingModule} from './login-routing.module';
import {LoginComponent} from './login.component';
import {MatIconModule} from "@angular/material/icon";
import {MatListModule} from "@angular/material/list";
import {MatButtonModule} from "@angular/material/button";
import {MatToolbarModule} from "@angular/material/toolbar";
import {MatRippleModule} from "@angular/material/core";
import { AdminComponent } from './admin/admin.component';
import { UserComponent } from './user/user.component';
import { ReactiveFormsModule } from '@angular/forms';
import { AdminExtNextComponent } from './admin-ext-next/admin-ext-next.component';
import { AdminChoiceComponent } from './admin-choice/admin-choice.component';
import { AdminActivateOtpComponent } from './admin-activate-otp/admin-activate-otp.component';
import { AdminSendOtpComponent } from './admin-send-otp/admin-send-otp.component';


@NgModule({
  declarations: [
    LoginComponent,
    AdminComponent,
    UserComponent,
    AdminExtNextComponent,
    AdminChoiceComponent,
    AdminActivateOtpComponent,
    AdminSendOtpComponent,
  ],
  imports: [
    CommonModule,
    LoginRoutingModule,
    MatIconModule,
    MatListModule,
    MatButtonModule,
    MatToolbarModule,
    MatRippleModule,
    ReactiveFormsModule
  ]
})
export class LoginModule { }
