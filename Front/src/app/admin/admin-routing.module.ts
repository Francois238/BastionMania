import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AdminComponent } from './admin.component';
import { MenuComponent } from './menu/menu.component';
import { ListAdminComponent } from './list-admin/list-admin.component';
import { ProfilComponent } from './profil/profil.component';

const routes: Routes = [
  { path: 'menu', component: MenuComponent },
  { path: 'admins', component: ListAdminComponent },
  { path: 'profil', component: ProfilComponent }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AdminRoutingModule { }
