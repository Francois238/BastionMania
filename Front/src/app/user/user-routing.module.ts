import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { UserComponent } from './user.component';
import { MenuComponent } from './menu/menu.component';
import {BastionsComponent} from "./bastions/bastions.component";
import {BastiondetailComponent} from "./bastiondetail/bastiondetail.component";

const routes: Routes = [
  { path: 'menu', component: MenuComponent },
  { path: 'user/bastions', component: BastionsComponent },
  { path: 'user/bastions/:bastion_id', component: BastiondetailComponent },
  { path: '', component: UserComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class UserRoutingModule { }
