import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { BastionlistComponent } from './bastionlist.component';

const routes: Routes = [{ path: '', component: BastionlistComponent }];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class BastionlistRoutingModule { }
