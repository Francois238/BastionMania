import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { UserService } from '../user.service';
import { Router } from '@angular/router';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';

@Component({
  selector: 'app-bastiondetail',
  templateUrl: './bastiondetail.component.html',
  styleUrls: ['./bastiondetail.component.scss']
})
export class BastiondetailComponent {

  @Input() bastion!: BastionInfo;
  @Output() newItemEvent = new EventEmitter<string>();

  message : string =''

  constructor(protected router: Router) { }


  getDetail(){

    this.router.navigate([`/user/bastions/${this.bastion.bastion_id}`]);
  }
}