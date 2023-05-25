import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { UserComponent } from './user.component';
import { MenuComponent } from './menu/menu.component';
import {BastionsComponent} from "./bastions/bastions.component";
import {BastiondetailComponent} from "./bastiondetail/bastiondetail.component";
import { ProfilComponent } from './profil/profil.component';

const routes: Routes = [
  {
    path: '',
    component: UserComponent,
    children: [
      { path: '', redirectTo: 'menu', pathMatch: 'full' }, // Default route, redirect to 'menu'
      { path: 'menu', component: MenuComponent },
      { path: 'profil', component: ProfilComponent },
      { path: 'bastions', component: BastionsComponent },
      { path: 'bastions/:bastion_id', component: BastiondetailComponent },
    ]
  }
];


@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class UserRoutingModule { }
