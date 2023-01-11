import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AuthGuard } from './core/auth/auth.guard';
import { AuthService } from './core/auth/auth.service';

const routes: Routes = [
  {
    path: '',
    redirectTo: 'bastionlist',
    pathMatch: 'full',
  },
  {
    path: 'login',
    loadChildren: () =>
      import('./login/login.module').then((m) => m.LoginModule),
  },
  {
    path: 'bastionlist',
    /*runGuardsAndResolvers: 'always',
    data: { role: AuthService.roles.ADMIN },
    canActivate: [AuthGuard],
    canLoad: [AuthGuard],*/
    loadChildren: () =>
      import('./bastionlist/bastionlist.module').then(
        (m) => m.BastionlistModule
      ),
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
