import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { BastionlistRoutingModule } from './bastionlist-routing.module';
import { BastionlistComponent } from './bastionlist.component';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';
import { NgxDatatableModule } from '@swimlane/ngx-datatable';

@NgModule({
  declarations: [BastionlistComponent],
  imports: [
    CommonModule,
    BastionlistRoutingModule,
    NgxDatatableModule,
    NgbModule,
  ],
})
export class BastionlistModule {}
