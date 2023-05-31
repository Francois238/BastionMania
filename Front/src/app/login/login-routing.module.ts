import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { LoginComponent } from './login.component';
import { AdminComponent } from './admin/admin.component';
import { UserComponent } from './user/user.component';
import { AdminExtNextComponent } from './admin-ext-next/admin-ext-next.component';
import { AdminSendOtpComponent } from './admin-send-otp/admin-send-otp.component';
import { AdminActivateOtpComponent } from './admin-activate-otp/admin-activate-otp.component';
import { UserSendOtpComponent } from './user-send-otp/user-send-otp.component';
import { UserExtNextComponent } from './user-ext-next/user-ext-next.component';
import { InstallationComponent } from './installation/installation.component';
import { UserActivateOtpComponent } from './user-activate-otp/user-activate-otp.component';

const routes: Routes = [
  { path: 'admin', component: AdminComponent },
  { path: 'admin/extern/next', component: AdminExtNextComponent },
  { path: 'admin/otp', component: AdminSendOtpComponent },
  { path: 'admin/activate_otp', component: AdminActivateOtpComponent },
  { path: 'installation', component: InstallationComponent},
  { path: 'extern/next', component: UserExtNextComponent},
  { path: 'otp', component: UserSendOtpComponent},
  { path: 'activate_otp', component: UserActivateOtpComponent },
  { path: '', component: UserComponent},


];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class LoginRoutingModule { }
