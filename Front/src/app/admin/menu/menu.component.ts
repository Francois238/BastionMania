import { Component } from '@angular/core';
import { AdminService } from '../admin.service';

@Component({
  selector: 'app-menu',
  templateUrl: './menu.component.html',
  styleUrls: ['./menu.component.scss']
})
export class MenuComponent {

  constructor(private adminService: AdminService) {

    //this.adminService.validate_token();

  }
}
